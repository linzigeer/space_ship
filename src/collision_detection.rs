use crate::asteroid::Asteroid;
use crate::damage::Damage;
use crate::health::Health;
use crate::missile::Missile;
use crate::schedule::InGameSet;
use crate::spaceship::Spaceship;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection_then_store.in_set(InGameSet::CollectionDetect),
        )
        .add_systems(
            Update,
            (
                (
                    send_event_when_collisions_occurred::<Asteroid>,
                    send_event_when_collisions_occurred::<Spaceship>,
                    send_event_when_collisions_occurred::<Missile>,
                ),
                decrease_health_when_received_collision_event,
            ).chain()
                .in_set(InGameSet::EntityUpdate),
        ).add_event::<CollisionEvent>();
    }
}

fn collision_detection_then_store(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities = HashMap::new();
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a!= entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert(vec![])
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend_from_slice(collisions.as_slice());
        }
    }
}

fn send_event_when_collisions_occurred<T: Component>(
    mut event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            event_writer.send(CollisionEvent::new(entity, collided_entity));
        }
    }
}

fn decrease_health_when_received_collision_event(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&Damage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collision_event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            return;
        };
        let Ok(damage) = collision_damage_query.get(collided_entity) else {
            return;
        };
        health.value -= damage.damage;
    }
}
