use serde::Deserialize;

pub const MAP_WIDTH: u16 = 40;
pub const MAP_HEIGHT: u16 = 20;

#[derive(Deserialize, Debug, Clone)]
pub struct ProtoProjectile {
    pub x: u16,
    pub y: u16,
    pub pattern: Vec<(i8, i8)>,
}

pub struct Player {
    pub x: u16,
    pub y: u16,
    pub max_hp: u16,
    pub hp: u16,
}

pub struct Projectile {
    pub x: u16,
    pub y: u16,
    pub pattern: Vec<(i8, i8)>,
    pub step: usize,
    pub active: bool,
}