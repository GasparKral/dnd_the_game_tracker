use shared::api_types::inventory::{Currency, InventoryItem, UpdateCurrencyRequest, UpdateItemRequest};
use shared::persistence::{CampaignFile, SavedCharacter};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// PersistenceManager
// ---------------------------------------------------------------------------

/// Gestiona la lectura y escritura de la campaña en disco.
///
/// La campaña se guarda en un único archivo JSON:
///   `<data_dir>/campaign.json`
///
/// El directorio por defecto es:
///   - Linux/Mac: `~/.local/share/dnd-dm/`
///   - Windows:   `%APPDATA%\dnd-dm\`
#[derive(Debug)]
pub struct PersistenceManager {
    campaign_path: PathBuf,
    /// Campaña activa en memoria
    campaign: RwLock<Option<CampaignFile>>,
}

impl PersistenceManager {
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        let path = data_dir.as_ref().join("campaign.json");
        Self {
            campaign_path: path,
            campaign: RwLock::new(None),
        }
    }

    /// Crea el directorio de datos si no existe.
    pub async fn ensure_dir(&self) -> Result<(), PersistenceError> {
        if let Some(parent) = self.campaign_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| PersistenceError::Io(e.to_string()))?;
        }
        Ok(())
    }

    // ── Campaña ───────────────────────────────────────────────────────────────

    /// Carga la campaña desde disco. Si no existe, devuelve None.
    pub async fn load(&self) -> Result<Option<CampaignFile>, PersistenceError> {
        if !self.campaign_path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&self.campaign_path)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;

        let campaign: CampaignFile =
            serde_json::from_str(&raw).map_err(|e| PersistenceError::Parse(e.to_string()))?;

        info!("Campaña '{}' cargada desde disco.", campaign.name);
        *self.campaign.write().await = Some(campaign.clone());
        Ok(Some(campaign))
    }

    /// Crea una nueva campaña y la persiste en disco.
    pub async fn create(
        &self,
        name: String,
        description: String,
    ) -> Result<CampaignFile, PersistenceError> {
        self.ensure_dir().await?;
        let campaign = CampaignFile::new(name, description);
        self.write(&campaign).await?;
        *self.campaign.write().await = Some(campaign.clone());
        info!("Campaña '{}' creada.", campaign.name);
        Ok(campaign)
    }

    /// Devuelve la campaña activa en memoria, sin tocar disco.
    pub async fn current(&self) -> Option<CampaignFile> {
        self.campaign.read().await.clone()
    }

    /// true si hay una campaña cargada en memoria.
    pub async fn is_loaded(&self) -> bool {
        self.campaign.read().await.is_some()
    }

    // ── Personajes ────────────────────────────────────────────────────────────

    /// Añade o actualiza un personaje en la campaña activa y guarda en disco.
    pub async fn upsert_character(
        &self,
        character: SavedCharacter,
    ) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock
            .as_mut()
            .ok_or(PersistenceError::NoCampaign)?;

        info!(
            "Guardando personaje '{}' (id: {}) en campaña '{}'.",
            character.name, character.id, campaign.name
        );

        campaign.characters.insert(character.id, character);
        campaign.touch();

        self.write(campaign).await
    }

    /// Devuelve todos los personajes de la campaña activa.
    pub async fn all_characters(&self) -> Result<Vec<SavedCharacter>, PersistenceError> {
        let lock = self.campaign.read().await;
        let campaign = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        Ok(campaign.characters.values().cloned().collect())
    }

    /// Devuelve los personajes de un jugador concreto.
    pub async fn characters_by_player(
        &self,
        player_name: &str,
    ) -> Result<Vec<SavedCharacter>, PersistenceError> {
        let all = self.all_characters().await?;
        Ok(all
            .into_iter()
            .filter(|c| c.player_name.eq_ignore_ascii_case(player_name))
            .collect())
    }

    /// Devuelve un personaje por su id.
    pub async fn get_character(&self, id: Uuid) -> Result<Option<SavedCharacter>, PersistenceError> {
        let lock = self.campaign.read().await;
        let campaign = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        Ok(campaign.characters.get(&id).cloned())
    }

    // ── Inventario ────────────────────────────────────────────────────────────

    pub async fn get_inventory(&self, character_id: Uuid) -> Result<(Vec<InventoryItem>, Currency), PersistenceError> {
        let lock = self.campaign.read().await;
        let c = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        let ch = c.characters.get(&character_id).ok_or(PersistenceError::NoCampaign)?;
        Ok((ch.inventory.clone(), ch.currency.clone()))
    }

    pub async fn add_item(&self, character_id: Uuid, item: InventoryItem) -> Result<InventoryItem, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NoCampaign)?;
        ch.inventory.push(item.clone());
        campaign.touch();
        self.write(campaign).await?;
        Ok(item)
    }

    pub async fn update_item(&self, character_id: Uuid, item_id: Uuid, req: UpdateItemRequest) -> Result<InventoryItem, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NoCampaign)?;
        let item = ch.inventory.iter_mut()
            .find(|i| i.id == item_id)
            .ok_or(PersistenceError::NotFound)?;
        if let Some(q) = req.quantity { item.quantity = q; }
        if let Some(e) = req.equipped { item.equipped = e; }
        if let Some(n) = req.notes    { item.notes = n; }
        let result = item.clone();
        campaign.touch();
        self.write(campaign).await?;
        Ok(result)
    }

    pub async fn delete_item(&self, character_id: Uuid, item_id: Uuid) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NoCampaign)?;
        let before = ch.inventory.len();
        ch.inventory.retain(|i| i.id != item_id);
        if ch.inventory.len() == before { return Err(PersistenceError::NotFound); }
        campaign.touch();
        self.write(campaign).await
    }

    pub async fn update_currency(&self, character_id: Uuid, req: UpdateCurrencyRequest) -> Result<Currency, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NoCampaign)?;
        if let Some(v) = req.copper   { ch.currency.copper   = v; }
        if let Some(v) = req.silver   { ch.currency.silver   = v; }
        if let Some(v) = req.electrum { ch.currency.electrum = v; }
        if let Some(v) = req.gold     { ch.currency.gold     = v; }
        if let Some(v) = req.platinum { ch.currency.platinum = v; }
        let result = ch.currency.clone();
        campaign.touch();
        self.write(campaign).await?;
        Ok(result)
    }

    /// Actualiza el vault_path de la campaña activa.
    pub async fn set_vault_path(&self, path: String) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        campaign.vault_path = Some(path);
        campaign.touch();
        self.write(campaign).await
    }

    // ── IO ────────────────────────────────────────────────────────────────────

    async fn write(&self, campaign: &CampaignFile) -> Result<(), PersistenceError> {
        let json = serde_json::to_string_pretty(campaign)
            .map_err(|e| PersistenceError::Parse(e.to_string()))?;

        // Escritura atómica: escribir a .tmp y renombrar
        let tmp = self.campaign_path.with_extension("json.tmp");
        fs::write(&tmp, &json)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;
        fs::rename(&tmp, &self.campaign_path)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Error de E/S: {0}")]
    Io(String),
    #[error("Error de serialización: {0}")]
    Parse(String),
    #[error("No hay ninguna campaña cargada")]
    NoCampaign,
    #[error("Recurso no encontrado")]
    NotFound,
}
