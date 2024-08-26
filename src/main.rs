mod assets_loader;
mod asteroid;
mod camera;
mod debug;
mod movements;
mod spaceship;

use crate::assets_loader::AssetLoadPlugin;
use crate::asteroid::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::movements::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 750.,
        })
        .add_plugins(DebugPlugin)
        .add_plugins(AssetLoadPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
