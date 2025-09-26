use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::HUDAction, app_state::AppState};


pub fn app_state_contol(
    mut next_state: ResMut<NextState<AppState>>,
    action_state: Res<ActionState<HUDAction>>,
) {
    if action_state.just_pressed(&HUDAction::ToMenu) {
        next_state.set(AppState::Menu);
    }
    if action_state.just_pressed(&HUDAction::EnterLevel) {
        next_state.set(AppState::Play);
    }
}

pub fn print_state_change(
    state: Res<State<AppState>>
) {
    println!("state -> {:?}", *state);
}
