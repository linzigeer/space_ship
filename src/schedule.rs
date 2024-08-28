use bevy::prelude::*;

#[derive(SystemSet, Hash, Debug, Clone, PartialEq, Eq)]
pub enum InGameSet {
    UserInput,
    EntityUpdate,
    CollectionDetect,
    EntityDespawn,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::EntityDespawn,
                InGameSet::UserInput,
                InGameSet::EntityUpdate,
                InGameSet::CollectionDetect,
            )
                .chain(),
        );
        // .add_systems(
        //     Update,
        //     apply_deferred
        //         .after(InGameSet::EntityDespawn)
        //         .before(InGameSet::UserInput),
        // );
    }
}
