use bevy::{input::InputSystem, prelude::*};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

mod hud;

pub use hud::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlSet;


pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Common control set
            .configure_sets(
                Update,
                ControlSet
                // Conditional execution based on input manager
                .in_set(InputManagerSystem::ManualControl) // Part of manual control system
                .after(InputManagerSystem::Tick) // Order system after tick
                .after(InputManagerSystem::Update) // Order system after update
                .after(InputSystem), // Order system after general input system
            )

            // ================ HUD =============
            .add_plugins(InputManagerPlugin::<HUDAction>::default())
            .init_resource::<ActionState<HUDAction>>()
            .insert_resource(HUDAction::default_control_map())

            .add_systems(
                Update,
                handle_hud_controls
                .after(ControlSet)
            )
        ;
    }
}
