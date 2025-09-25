use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::*, buttons::components::{ButtonStyle, SimpleButton}};


pub struct SimplePlugin;
impl Plugin for SimplePlugin {
    fn build (&self, app: &mut App) {
        app
            // Recieve messages from view (button clicks)
            .add_systems(
                Update,
                simple_input_system
                .in_set(ControlSet)
            )
        ;
    }
}


pub fn simple_input_system(
    interaction_query: Query<(Entity, &Interaction, &SimpleButton, &ButtonStyle), Changed<Interaction>>,
    mut action_state: ResMut<ActionState<HUDAction>>,
    mut commands: Commands
) {
    for (entity, interaction, button_action, button) in &interaction_query {
        // Compute state transition but do not apply yet
        let next_state = match interaction {
            Interaction::Pressed => ButtonStyle::Active,
            Interaction::Hovered => ButtonStyle::Hover,
            Interaction::None => ButtonStyle::Idle,
        };

        // Emit some actions
        match *button {
            ButtonStyle::Active => match next_state {
                ButtonStyle::Hover => {
                    println!("press {:?}", button_action);
                    action_state.press(&button_action.action);
                },
                _ => {}
            },
            _ => {
                action_state.release(&button_action.action);
            },
        };

        // Apply state transition
        commands.entity(entity).insert(next_state);
    }
}
