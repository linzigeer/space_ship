mod assets_loader;
mod asteroid;
mod camera;
mod collision_detection;
mod damage;
mod debug;
mod despawn;
mod health;
mod missile;
mod movements;
mod schedule;
mod spaceship;
mod state;

use crate::assets_loader::AssetLoadPlugin;
use crate::asteroid::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionDetectionPlugin;
use crate::debug::DebugPlugin;
use crate::movements::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::state::GameStatePlugin;
use bevy::prelude::*;
use despawn::DespawnPlugin;
use schedule::SchedulePlugin;

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
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(CameraPlugin)
        .run();
}
