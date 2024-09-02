use bevy::prelude::*;

#[derive(States, Debug, Hash, Default, PartialEq, Eq, Copy, Clone)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            (
                set_pause_or_continue_game_state,
                set_restart_game_state.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}

fn set_pause_or_continue_game_state(
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_game_state.get() {
            GameState::InGame => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::InGame),
            _ => (),
        }
    }
}

fn set_restart_game_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
