use crate::api_types::catalog::{CatalogEntry, TraitDetail};
use crate::models::character::Player;
use std::fmt::Debug;

/// Implementa los tres métodos del trait `Feat` para structs ZST simples.
///
/// Uso básico (sin choices):
/// ```
/// simple_feat!(
///     Alert, "alert", "Alerta",
///     description: "+5 iniciativa. No puedes ser sorprendido.",
///     traits_preview: ["Sin sorpresas", "+5 Iniciativa"],
///     traits_detail: [
///         ("Sin sorpresas", "No puedes ser sorprendido ni los ocultos tienen ventaja al atacarte."),
///         ("+5 Iniciativa",  "Sumas +5 a tu tirada de iniciativa."),
///     ],
/// );
/// ```
///
/// Con choices: añade `choices: [...]` y `required_choices: ["id"]`
/// como parámetros opcionales al final.
macro_rules! simple_feat {
    // ── variante sin choices ─────────────────────────────────────────────────
    (
        $type:ident, $id:expr, $name:expr,
        description: $desc:expr,
        traits_preview: [$($preview:expr),* $(,)?],
        traits_detail:  [$(($td_name:expr, $td_desc:expr)),* $(,)?]
        $(,)?
    ) => {
        impl $crate::traits::feat::Feat for $type {
            fn id(&self) -> &'static str { $id }

            fn catalog_entry(&self) -> $crate::api_types::catalog::CatalogEntry {
                $crate::api_types::catalog::CatalogEntry {
                    id:               $id.into(),
                    name:             $name.into(),
                    source:           "PHB2024".into(),
                    description:      Some($desc.into()),
                    lore:             None,
                    image_url:        None,
                    choices:          vec![],
                    required_choices: vec![],
                    traits_preview:   vec![$($preview.into()),*],
                    traits_detail:    vec![$(
                        $crate::api_types::catalog::TraitDetail::new($td_name, $td_desc)
                    ),*],
                    speed_m:          None,
                    size:             None,
                }
            }

            fn apply(&self, _c: &mut $crate::models::character::Player) {}
        }
    };

    // ── variante con choices ─────────────────────────────────────────────────
    (
        $type:ident, $id:expr, $name:expr,
        description:      $desc:expr,
        traits_preview:   [$($preview:expr),* $(,)?],
        traits_detail:    [$(($td_name:expr, $td_desc:expr)),* $(,)?],
        choices:          [$($choice:expr),* $(,)?],
        required_choices: [$($req:expr),* $(,)?]
        $(,)?
    ) => {
        impl $crate::traits::feat::Feat for $type {
            fn id(&self) -> &'static str { $id }

            fn catalog_entry(&self) -> $crate::api_types::catalog::CatalogEntry {
                $crate::api_types::catalog::CatalogEntry {
                    id:               $id.into(),
                    name:             $name.into(),
                    source:           "PHB2024".into(),
                    description:      Some($desc.into()),
                    lore:             None,
                    image_url:        None,
                    choices:          vec![$($choice),*],
                    required_choices: vec![$($req.into()),*],
                    traits_preview:   vec![$($preview.into()),*],
                    traits_detail:    vec![$(
                        $crate::api_types::catalog::TraitDetail::new($td_name, $td_desc)
                    ),*],
                    speed_m:          None,
                    size:             None,
                }
            }

            fn apply(&self, _c: &mut $crate::models::character::Player) {}
        }
    };
}

pub(crate) use simple_feat;

pub trait Feat: Debug + Send + Sync {
    fn id(&self) -> &'static str;
    fn catalog_entry(&self) -> CatalogEntry;
    fn apply(&self, character: &mut Player);
}
