use crate::health::Health;
use crate::schedule::InGameSet;
use bevy::prelude::*;
use crate::spaceship::Spaceship;
use crate::state::GameState;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::EntityDespawn),
        ).add_systems(OnEnter(GameState::GameOver), despawn_all_entities_before_game_restart);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities_before_game_restart(mut commands: Commands, query: Query<Entity, (With<Health>, Without<Spaceship>)>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}