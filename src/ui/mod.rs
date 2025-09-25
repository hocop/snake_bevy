mod components;
pub mod elements;
mod view;
mod ui_state;

use bevy::prelude::*;

pub use ui_state::*;
use view::*;

use crate::{actions::ControlSet, app_state::AppState};


pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build (&self, app: &mut App) {
        // Try to follow ELM architecture
        app
            // Update the state based on messages
            .add_systems(
                Update,
                app_state_contol
                .after(ControlSet)
            )

            // Build the view based on model
            .add_systems(
                Update,
                (print_state_change, clear_all_ui, ui_view_system).chain()
                .run_if(state_changed::<AppState>)
            )
        ;
    }
}
