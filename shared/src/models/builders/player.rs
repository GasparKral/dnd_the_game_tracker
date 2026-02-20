use super::super::attributes::{Attribute, HitPoints};
use std::marker::PhantomData;

// --- Typestates para campos obligatorios ---
pub struct NoName;
pub struct WithName(String);

pub struct NoHp;
pub struct WithHp(HitPoints);

pub struct NoAttributes;
pub struct WithAttributes([Attribute; 6]);
