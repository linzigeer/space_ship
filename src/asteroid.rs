use crate::assets_loader::SceneAssets;
use crate::{
    collision_detection::Collider,
    movements::{Acceleration, MovingObjectBundle, Velocity},
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Range;
use crate::schedule::InGameSet;

const ASTEROID_TRANSLATION_X: Range<f32> = -25.0..25.0;
const ASTEROID_TRANSLATION_Z: Range<f32> = 0.0..25.0;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 5.0;
const ASTEROID_SPAWN_INTERVAL: f32 = 1.0;
const ASTEROIDS_ROTATE_SPEED: f32 = 2.5;
const ASTEROIDS_COLLISION_RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_INTERVAL, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids).in_set(InGameSet::EntityUpdate),
        );
    }
}

/**
* 在一个时钟周期完成后生成个小行星
*/
fn spawn_asteroid(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    //time在内部累计时钟周期
    timer.timer.tick(time.delta());
    //如果一个时钟周期还没结束，则终止生成流程（不生成小行星）
    if !timer.timer.just_finished() {
        return;
    }

    let mut rng = thread_rng();
    //小行星生成的随机位置
    let translation = Vec3::new(
        rng.gen_range(ASTEROID_TRANSLATION_X),
        0.0,
        rng.gen_range(ASTEROID_TRANSLATION_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.00)).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            collider: Collider::new(ASTEROIDS_COLLISION_RADIUS),
            scene_bundle: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ASTEROIDS_ROTATE_SPEED * time.delta_seconds());
    }
}