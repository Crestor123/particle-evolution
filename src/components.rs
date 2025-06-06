use bevy::prelude::*;

#[derive(Component)]
pub struct Charge(pub i8);

#[derive(Component)]
pub struct Particle {
    pub rotation: i8,
    pub vibration: i8,
    pub positive: bool,
    pub group: Entity,
    pub bonds: Vec<Entity>
}

#[derive(Component)]
pub struct Group {
    pub name: String,
    pub radius: f32,
}

#[derive(Component)]
pub struct Bond {
    pub particle_a: Entity,
    pub particle_b: Entity,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);