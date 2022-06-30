use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Playing,
    GameOver,
}

pub fn reset_state(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        app_state.push(AppState::Playing).unwrap();
    }
}
