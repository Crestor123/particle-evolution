use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub rotation: i8,
    pub vibration: i8,
    pub group: Entity
}

#[derive(Component)]
pub struct Group{
    pub name: String,
    pub charge: i8,
    pub radius: f32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);