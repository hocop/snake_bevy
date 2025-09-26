use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum HUDAction {
    Placeholder,

    // Global switching
    EnterLevel,
    ToMenu,
    ExitGame,

    // In game
    GoLeft,
    GoRight,
}


impl HUDAction {
    pub fn default_control_map() -> InputMap<HUDAction> {
        InputMap::default()
            .with(HUDAction::ExitGame, KeyCode::KeyQ)
            .with(HUDAction::ToMenu, KeyCode::Escape)
            .with(HUDAction::GoLeft, KeyCode::ArrowLeft)
            .with(HUDAction::GoRight, KeyCode::ArrowRight)
    }
}


pub fn handle_hud_controls(action_state: Res<ActionState<HUDAction>>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    if action_state.just_pressed(&HUDAction::ExitGame) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}
