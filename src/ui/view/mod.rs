use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::HUDAction, buttons::components::SimpleButton};

use super::components::*;
use crate::app_state::AppState;


mod menu;

use menu::*;


/// View function - the ELM way
pub fn ui_view_system(
    current_state: Res<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Only rebuild when state changed
    let view = match current_state.get() {
        AppState::Menu => commands.spawn(build_menu()),
        AppState::Play => todo!()
    }.id();
    commands.entity(view).insert(UIRoot {});
}


fn horizontal_spacer() -> Node {
    Node {  width: Val::Px(5.), ..default() }
}


pub fn clear_all_ui(
    mut commands: Commands,
    ui_roots: Query<Entity, With<UIRoot>>,
    buttons: Query<&SimpleButton>,
    mut action_state: ResMut<ActionState<HUDAction>>,
) {
    for view in &ui_roots {
        // Delete views
        commands.entity(view).despawn();
    }

    // Release all buttons
    for simple_button in &buttons {
        if action_state.pressed(&simple_button.action) {
            println!("release on delete {simple_button:?}");
            action_state.release(&simple_button.action);
        }
    }
}
