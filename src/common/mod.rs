use bevy::prelude::*;

#[derive(Component)]
pub enum HDirection {
    Left,
    Right,
}

#[derive(Component)]
pub enum VDirection {
    Up,
    Down,
}

pub const SPEED: f32 = 10.0;
pub const RADIUS: f32 = 100.0;

pub const BOXWIDTH: f32 = 700.0;
pub const BOXHEIGHT: f32 = 512.0;

/// Marker component for things we spawn once and never despawn
#[derive(Component)]
pub struct Immortal;


#[derive(Component)]
pub struct FirstPassEntity;

#[derive(Component)]
pub struct ModuleWin;