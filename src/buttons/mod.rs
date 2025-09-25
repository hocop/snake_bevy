use bevy::prelude::*;

use crate::{actions::ControlSet, buttons::resources::AnyButtonHovered};

pub mod components;
pub mod toggle;
pub mod choice;
pub mod systems;
pub mod resources;
pub mod simple;

use systems::*;
use choice::*;
use toggle::*;
use simple::*;


pub struct ButtonsPlugin;
impl Plugin for ButtonsPlugin {
    fn build (&self, app: &mut App) {
        app
            .init_resource::<AnyButtonHovered>()

            // Recieve messages from view (button clicks)
            .add_systems(
                Update,
                (
                    // Informative systems
                    update_hovering_state,
                    // Style
                    button_style,
                ).chain()
                .in_set(ControlSet)
            )

            // Add simple buttons
            .add_plugins(SimplePlugin)
            // Add toggles
            .add_plugins(TogglePlugin)
            // Add all multiple choices
        ;
    }
}
