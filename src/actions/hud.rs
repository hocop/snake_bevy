use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::buttons::components::Toggle;


#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum HUDAction {
    Placeholder,

    // Global switching
    EnterLevel,
    ToMenu,
    ExitGame,

    // Inside editor
    Clean,

    // In game
    StartSim,
    StopSim,
    Toggle(Toggle),

    // Actions emitted by the simulation
    MissingPaths,
}


impl HUDAction {
    pub fn default_control_map() -> InputMap<HUDAction> {
        InputMap::default()
            .with(HUDAction::ExitGame, KeyCode::KeyQ)
    }
}


pub fn handle_hud_controls(action_state: Res<ActionState<HUDAction>>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    if action_state.just_pressed(&HUDAction::ExitGame) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}
