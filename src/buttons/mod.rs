use bevy::prelude::*;

use crate::{actions::ControlSet, buttons::resources::AnyButtonHovered};

pub mod components;
pub mod systems;
pub mod resources;
pub mod simple;

use systems::*;
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
        ;
    }
}
