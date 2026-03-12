use crate::api_types::catalog::CatalogEntry;
use crate::models::character::Player;
use std::fmt::Debug;

pub trait Background: Debug + Send + Sync {
    fn id(&self) -> &'static str;
    fn catalog_entry(&self) -> CatalogEntry;
    fn apply(&self, character: &mut Player);
}
