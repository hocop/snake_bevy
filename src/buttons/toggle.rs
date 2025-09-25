use bevy::prelude::*;

use crate::{actions::ControlSet, buttons::components::*};


pub struct TogglePlugin;
impl Plugin for TogglePlugin {
    fn build (&self, app: &mut App) {
        app
            // State of all toggles (fast forward, etc.)
            .init_state::<ToggleState>()
            .add_observer(sync_new_toggle_to_state)

            // Recieve messages from view (button clicks)
            .add_systems(
                Update,
                toggle_input_system
                .in_set(ControlSet)
            )
        ;
    }
}


/// State of all toggles in the game
#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct ToggleState(u32);

impl ToggleState {
    pub fn toggle(&self, toggle: Toggle) -> Self {
        ToggleState(self.0 ^ (1 >> toggle as usize))
    }

    pub fn is_on(&self, toggle: Toggle) -> bool {
        self.0 & (1 >> toggle as usize) != 0
    }

    pub fn with_value(&self, toggle: Toggle, value: bool) -> Self {
        if value == self.is_on(toggle) {
            *self
        } else {
            self.toggle(toggle)
        }
    }
}

pub fn sync_new_toggle_to_state(
    trigger: Trigger<OnInsert, Toggle>,
    toggles: Query<&Toggle>,
    toggle_state: Res<State<ToggleState>>,
    mut commands: Commands,
) {
    let toggle = toggles.get(trigger.target()).unwrap();
    if toggle_state.get().is_on(*toggle) {
        commands.entity(trigger.target()).insert(ButtonStyle::Active);
    }
}

pub fn toggle_input_system(
    interaction_query: Query<(Entity, &Interaction, &Toggle, &ButtonStyle), Changed<Interaction>>,
    toggle_state: Res<State<ToggleState>>,
    mut next_toggle_state: ResMut<NextState<ToggleState>>,
    mut commands: Commands
) {
    for (entity, interaction, toggle, button) in &interaction_query {
        // Compute state transition but do not apply yet
        let next_state = match button {
            ButtonStyle::Idle => match interaction {
                Interaction::Pressed => ButtonStyle::Active,
                Interaction::Hovered => ButtonStyle::Hover,
                Interaction::None => ButtonStyle::Idle,
            },
            ButtonStyle::Active => match interaction {
                Interaction::Pressed => ButtonStyle::Hover,
                _ => ButtonStyle::Active,
            },
            ButtonStyle::Hover => match interaction {
                Interaction::Pressed => ButtonStyle::Active,
                Interaction::Hovered => ButtonStyle::Hover,
                Interaction::None => ButtonStyle::Idle,
            },
        };

        // Change toggle state
        next_toggle_state.set(toggle_state.get().with_value(*toggle, next_state == ButtonStyle::Active));

        // Apply state transition
        commands.entity(entity).insert(next_state);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut toggle_state = ToggleState::default();
        assert_eq!(toggle_state.is_on(Toggle::FastForward), false);
        assert_eq!(toggle_state.is_on(Toggle::SomeOtherToggle), false);

        toggle_state = toggle_state.toggle(Toggle::FastForward);
        assert_eq!(toggle_state.is_on(Toggle::FastForward), true);
        assert_eq!(toggle_state.is_on(Toggle::SomeOtherToggle), false);

        toggle_state = toggle_state.toggle(Toggle::FastForward);
        assert_eq!(toggle_state.is_on(Toggle::FastForward), false);
        assert_eq!(toggle_state.is_on(Toggle::SomeOtherToggle), false);
    }
}
