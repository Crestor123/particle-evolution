use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use bevy::window::WindowResolution;

mod particle;
mod components;
pub use components::*;
mod resources;
pub use resources::*;

use crate::particle::ParticlePlugin;

const X_EXTENTS: f32 = 1280.0;
const Y_EXTENTS: f32 = 720.0;

const FRICTION: f32 = 0.6;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
            title: "Particle Evolution".into(),
            resolution: WindowResolution::new(X_EXTENTS, Y_EXTENTS).with_scale_factor_override(1.0),
            ..default()
            }),
            ..default()
        }),
        Wireframe2dPlugin::default(),
        ParticlePlugin,
    ));
    app.insert_resource(Extents(Vec2::new(X_EXTENTS, Y_EXTENTS)));
    app.insert_resource(Friction(FRICTION));
    app.run();
}