use shared::api_types::inventory::{Currency, InventoryItem, UpdateCurrencyRequest, UpdateItemRequest};
use shared::api_types::spells::{AddSpellRequest, Spell, SpellSlotLevel, SpellsResponse, UpdateSpellSlotsRequest};
use shared::persistence::{CampaignFile, SavedCharacter};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// PersistenceManager — multi-campaña
// ---------------------------------------------------------------------------
//
// Cada campaña se guarda en su propio archivo:
//   <data_dir>/campaigns/<nombre-slug>.json
//
// La campaña activa también se referencia en:
//   <data_dir>/active_campaign.txt  (contiene el slug/nombre del archivo)
//
// Directorio por defecto:
//   Linux/Mac: ~/.local/share/dnd-dm/
//   Windows:   %APPDATA%\dnd-dm\

#[derive(Debug)]
pub struct PersistenceManager {
    data_dir: PathBuf,
    /// Campaña activa en memoria
    campaign: RwLock<Option<CampaignFile>>,
}

impl PersistenceManager {
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            data_dir: data_dir.as_ref().to_path_buf(),
            campaign: RwLock::new(None),
        }
    }

    fn campaigns_dir(&self) -> PathBuf {
        self.data_dir.join("campaigns")
    }

    fn active_ptr(&self) -> PathBuf {
        self.data_dir.join("active_campaign.txt")
    }

    fn campaign_path_for(&self, filename: &str) -> PathBuf {
        self.campaigns_dir().join(filename)
    }

    /// Nombre de archivo desde el nombre de la campaña (slug seguro).
    fn slug(name: &str) -> String {
        let base: String = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' { c.to_ascii_lowercase() } else { '_' })
            .collect();
        format!("{}.json", if base.is_empty() { "campaign".to_string() } else { base })
    }

    /// Crea el directorio de datos si no existe.
    pub async fn ensure_dir(&self) -> Result<(), PersistenceError> {
        fs::create_dir_all(self.campaigns_dir())
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))
    }

    // ── Lista de campañas ─────────────────────────────────────────────────────

    /// Devuelve todas las campañas guardadas en disco (nombre, descripción, personajes, fecha).
    pub async fn list_campaigns(&self) -> Result<Vec<CampaignSummaryEntry>, PersistenceError> {
        let dir = self.campaigns_dir();
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut entries = fs::read_dir(&dir)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;

        let mut result = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| PersistenceError::Io(e.to_string()))? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(raw) = fs::read_to_string(&path).await {
                if let Ok(c) = serde_json::from_str::<CampaignFile>(&raw) {
                    result.push(CampaignSummaryEntry {
                        filename: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                        name: c.name,
                        description: c.description,
                        character_count: c.characters.len(),
                        updated_at: c.updated_at,
                    });
                }
            }
        }
        result.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(result)
    }

    // ── Campaña activa ────────────────────────────────────────────────────────

    /// Carga la última campaña activa. Primero lee active_campaign.txt; si no
    /// existe, intenta cargar el único archivo que haya (compatibilidad hacia atrás).
    pub async fn load(&self) -> Result<Option<CampaignFile>, PersistenceError> {
        self.ensure_dir().await?;

        // Intentar leer el puntero a campaña activa
        let filename = if self.active_ptr().exists() {
            fs::read_to_string(self.active_ptr())
                .await
                .map_err(|e| PersistenceError::Io(e.to_string()))?
                .trim()
                .to_string()
        } else {
            // Compatibilidad: buscar campaign.json en el data_dir antiguo
            let legacy = self.data_dir.join("campaign.json");
            if legacy.exists() {
                // Migrar: mover al nuevo directorio
                self.ensure_dir().await?;
                let raw = fs::read_to_string(&legacy).await.map_err(|e| PersistenceError::Io(e.to_string()))?;
                if let Ok(c) = serde_json::from_str::<CampaignFile>(&raw) {
                    let slug = Self::slug(&c.name);
                    let new_path = self.campaign_path_for(&slug);
                    fs::write(&new_path, &raw).await.map_err(|e| PersistenceError::Io(e.to_string()))?;
                    fs::write(self.active_ptr(), &slug).await.map_err(|e| PersistenceError::Io(e.to_string()))?;
                    let _ = fs::remove_file(&legacy).await;
                    info!("Campaña legacy '{}' migrada a campaigns/{}", c.name, slug);
                    slug
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        };

        let path = self.campaign_path_for(&filename);
        if !path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&path)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;
        let campaign: CampaignFile =
            serde_json::from_str(&raw).map_err(|e| PersistenceError::Parse(e.to_string()))?;

        info!("Campaña '{}' cargada desde disco.", campaign.name);
        *self.campaign.write().await = Some(campaign.clone());
        Ok(Some(campaign))
    }

    /// Carga una campaña específica por su filename y la activa.
    pub async fn load_campaign(&self, filename: &str) -> Result<CampaignFile, PersistenceError> {
        let path = self.campaign_path_for(filename);
        if !path.exists() {
            return Err(PersistenceError::NotFound);
        }
        let raw = fs::read_to_string(&path)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;
        let campaign: CampaignFile =
            serde_json::from_str(&raw).map_err(|e| PersistenceError::Parse(e.to_string()))?;

        // Actualizar puntero activo
        fs::write(self.active_ptr(), filename)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;
        *self.campaign.write().await = Some(campaign.clone());
        info!("Campaña '{}' activada.", campaign.name);
        Ok(campaign)
    }

    /// Elimina una campaña por su filename. Si era la activa, la descarga de memoria.
    pub async fn delete_campaign(&self, filename: &str) -> Result<(), PersistenceError> {
        let path = self.campaign_path_for(filename);
        if !path.exists() {
            return Err(PersistenceError::NotFound);
        }
        fs::remove_file(&path)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;

        // Si era la activa, limpiar puntero y memoria
        if self.active_ptr().exists() {
            let current = fs::read_to_string(self.active_ptr()).await.unwrap_or_default();
            if current.trim() == filename {
                let _ = fs::remove_file(self.active_ptr()).await;
                *self.campaign.write().await = None;
            }
        }
        info!("Campaña '{}' eliminada.", filename);
        Ok(())
    }

    /// Crea una nueva campaña y la persiste en disco, activándola.
    pub async fn create(
        &self,
        name: String,
        description: String,
    ) -> Result<CampaignFile, PersistenceError> {
        self.ensure_dir().await?;
        let campaign = CampaignFile::new(name.clone(), description);
        let slug = Self::slug(&name);
        self.write_to(&campaign, &slug).await?;
        // Marcar como activa
        fs::write(self.active_ptr(), &slug)
            .await
            .map_err(|e| PersistenceError::Io(e.to_string()))?;
        *self.campaign.write().await = Some(campaign.clone());
        info!("Campaña '{}' creada ({})", campaign.name, slug);
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

    pub async fn upsert_character(
        &self,
        character: SavedCharacter,
    ) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        info!("Guardando personaje '{}' (id: {}) en campaña '{}'.", character.name, character.id, campaign.name);

        // MERGE: preservar inventario, monedas y hechizos existentes si el payload
        // que llega no los incluye (evita que editar la ficha desde el DM borre
        // objetos añadidos por el jugador móvil).
        let merged = if let Some(existing) = campaign.characters.get(&character.id) {
            SavedCharacter {
                // Si el payload trae inventario vacío y el existente no lo está,
                // conservamos el existente.
                inventory: if character.inventory.is_empty() && !existing.inventory.is_empty() {
                    existing.inventory.clone()
                } else {
                    character.inventory
                },
                currency: if character.currency == Currency::default() && existing.currency != Currency::default() {
                    existing.currency.clone()
                } else {
                    character.currency
                },
                spell_slots: if character.spell_slots.is_empty() && !existing.spell_slots.is_empty() {
                    existing.spell_slots.clone()
                } else {
                    character.spell_slots
                },
                known_spells: if character.known_spells.is_empty() && !existing.known_spells.is_empty() {
                    existing.known_spells.clone()
                } else {
                    character.known_spells
                },
                prepared_spells: if character.prepared_spells.is_empty() && !existing.prepared_spells.is_empty() {
                    existing.prepared_spells.clone()
                } else {
                    character.prepared_spells
                },
                ..character
            }
        } else {
            character
        };

        campaign.characters.insert(merged.id, merged);
        campaign.touch();
        self.write(campaign).await
    }

    pub async fn all_characters(&self) -> Result<Vec<SavedCharacter>, PersistenceError> {
        let lock = self.campaign.read().await;
        let campaign = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        Ok(campaign.characters.values().cloned().collect())
    }

    pub async fn characters_by_player(
        &self,
        player_name: &str,
    ) -> Result<Vec<SavedCharacter>, PersistenceError> {
        let all = self.all_characters().await?;
        Ok(all.into_iter().filter(|c| c.player_name.eq_ignore_ascii_case(player_name)).collect())
    }

    pub async fn get_character(&self, id: Uuid) -> Result<Option<SavedCharacter>, PersistenceError> {
        let lock = self.campaign.read().await;
        let campaign = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        Ok(campaign.characters.get(&id).cloned())
    }

    // ── Inventario ────────────────────────────────────────────────────────────

    pub async fn get_inventory(&self, character_id: Uuid) -> Result<(Vec<InventoryItem>, Currency), PersistenceError> {
        let lock = self.campaign.read().await;
        let c = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        let ch = c.characters.get(&character_id).ok_or(PersistenceError::NotFound)?;
        Ok((ch.inventory.clone(), ch.currency.clone()))
    }

    pub async fn add_item(&self, character_id: Uuid, item: InventoryItem) -> Result<InventoryItem, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        ch.inventory.push(item.clone());
        campaign.touch();
        self.write(campaign).await?;
        Ok(item)
    }

    pub async fn update_item(&self, character_id: Uuid, item_id: Uuid, req: UpdateItemRequest) -> Result<InventoryItem, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        let item = ch.inventory.iter_mut().find(|i| i.id == item_id).ok_or(PersistenceError::NotFound)?;
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
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        let before = ch.inventory.len();
        ch.inventory.retain(|i| i.id != item_id);
        if ch.inventory.len() == before { return Err(PersistenceError::NotFound); }
        campaign.touch();
        self.write(campaign).await
    }

    pub async fn update_currency(&self, character_id: Uuid, req: UpdateCurrencyRequest) -> Result<Currency, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
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

    // ── Hechizos ──────────────────────────────────────────────────────────────

    pub async fn get_spells(&self, character_id: Uuid) -> Result<SpellsResponse, PersistenceError> {
        let lock = self.campaign.read().await;
        let c = lock.as_ref().ok_or(PersistenceError::NoCampaign)?;
        let ch = c.characters.get(&character_id).ok_or(PersistenceError::NotFound)?;
        Ok(SpellsResponse {
            known_spells: ch.known_spells.clone(),
            prepared_spells: ch.prepared_spells.clone(),
            spell_slots: ch.spell_slots.clone(),
        })
    }

    pub async fn add_known_spell(&self, character_id: Uuid, req: AddSpellRequest) -> Result<Spell, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        let mut spell = Spell::new(req.name, req.level);
        spell.school       = req.school;
        spell.casting_time = req.casting_time;
        spell.range        = req.range;
        spell.duration     = req.duration;
        spell.components   = req.components;
        spell.description  = req.description;
        spell.damage       = req.damage;
        spell.saving_throw = req.saving_throw;
        spell.notes        = req.notes;
        spell.concentration = req.concentration;
        spell.ritual       = req.ritual;
        ch.known_spells.push(spell.clone());
        if req.prepared {
            ch.prepared_spells.push(spell.clone());
        }
        campaign.touch();
        self.write(campaign).await?;
        Ok(spell)
    }

    pub async fn remove_known_spell(&self, character_id: Uuid, spell_id: Uuid) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        let before = ch.known_spells.len();
        ch.known_spells.retain(|s| s.id != spell_id);
        ch.prepared_spells.retain(|s| s.id != spell_id);
        if ch.known_spells.len() == before { return Err(PersistenceError::NotFound); }
        campaign.touch();
        self.write(campaign).await
    }

    pub async fn toggle_prepared_spell(&self, character_id: Uuid, spell_id: Uuid) -> Result<bool, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        let spell = ch.known_spells.iter().find(|s| s.id == spell_id).ok_or(PersistenceError::NotFound)?.clone();
        let is_prepared = ch.prepared_spells.iter().any(|s| s.id == spell_id);
        if is_prepared {
            ch.prepared_spells.retain(|s| s.id != spell_id);
        } else {
            ch.prepared_spells.push(spell);
        }
        let now_prepared = !is_prepared;
        campaign.touch();
        self.write(campaign).await?;
        Ok(now_prepared)
    }

    pub async fn update_spell_slots(&self, character_id: Uuid, req: UpdateSpellSlotsRequest) -> Result<Vec<SpellSlotLevel>, PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        let ch = campaign.characters.get_mut(&character_id).ok_or(PersistenceError::NotFound)?;
        ch.spell_slots = req.slots.clone();
        campaign.touch();
        self.write(campaign).await?;
        Ok(req.slots)
    }

    pub async fn set_vault_path(&self, path: String) -> Result<(), PersistenceError> {
        let mut lock = self.campaign.write().await;
        let campaign = lock.as_mut().ok_or(PersistenceError::NoCampaign)?;
        campaign.vault_path = Some(path);
        campaign.touch();
        self.write(campaign).await
    }

    // ── IO ────────────────────────────────────────────────────────────────────

    /// Escribe la campaña activa usando su nombre como slug.
    async fn write(&self, campaign: &CampaignFile) -> Result<(), PersistenceError> {
        let slug = Self::slug(&campaign.name);
        self.write_to(campaign, &slug).await
    }

    /// Escritura atómica a un archivo concreto.
    async fn write_to(&self, campaign: &CampaignFile, filename: &str) -> Result<(), PersistenceError> {
        let path = self.campaign_path_for(filename);
        let json = serde_json::to_string_pretty(campaign)
            .map_err(|e| PersistenceError::Parse(e.to_string()))?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, &json).await.map_err(|e| PersistenceError::Io(e.to_string()))?;
        fs::rename(&tmp, &path).await.map_err(|e| PersistenceError::Io(e.to_string()))?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Summary entry (para la lista de campañas)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
pub struct CampaignSummaryEntry {
    pub filename: String,
    pub name: String,
    pub description: String,
    pub character_count: usize,
    pub updated_at: String,
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
