use super::character::Entity;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Effect<T>: Debug {
    fn apply(&self, character: &mut Entity);
    fn end(&self, character: &mut Entity);

    // Clonar via trait object
    fn clone_box(&self) -> Box<dyn Effect<T>>;

    // Hash manual
    fn hash_code(&self) -> u64;
}

// Implementación de Clone para Box<dyn Effect>
impl<T> Clone for Box<dyn Effect<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Implementación de Hash para Box<dyn Effect>
impl<T> Hash for Box<dyn Effect<T>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash_code().hash(state);
    }
}
