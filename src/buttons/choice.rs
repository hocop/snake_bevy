use std::marker::PhantomData;

use bevy::prelude::*;

use crate::buttons::components::*;


pub trait StateConstraint = bevy::state::state::FreelyMutableState + Eq + Clone + Copy + Send + Sync + 'static;


#[derive(Default)]
pub struct MultiChoicePlugin<T: StateConstraint> {
    _marker: PhantomData<T>
}


impl<T: StateConstraint> Plugin for MultiChoicePlugin<T> {
    fn build (&self, app: &mut App) {
        app.
            add_systems(
                Update,
                (
                    choice_input_stage_1::<T>,
                    choice_input_stage_2::<T>.run_if(state_changed::<T>)
                )
            )
        ;
    }
}


pub fn choice_input_stage_1<T: StateConstraint>(
    interaction_query: Query<(Entity, &Interaction, &MultipleChoice<T>, &ButtonStyle), Changed<Interaction>>,
    mut next_choice_state: ResMut<NextState<T>>,
    mut commands: Commands
) {
    for (entity, interaction, choice, button) in &interaction_query {
        // Compute style transition but do not apply yet
        let next_state = match button {
            ButtonStyle::Idle => match interaction {
                Interaction::Pressed => ButtonStyle::Active,
                Interaction::Hovered => ButtonStyle::Hover,
                Interaction::None => ButtonStyle::Idle,
            },
            ButtonStyle::Active => ButtonStyle::Active,
            ButtonStyle::Hover => match interaction {
                Interaction::Pressed => ButtonStyle::Active,
                Interaction::Hovered => ButtonStyle::Hover,
                Interaction::None => ButtonStyle::Idle,
            },
        };

        // Change choice state
        match interaction {
            Interaction::Pressed => {
                next_choice_state.set(choice.choice);
            }
            _ => {}
        };

        // Apply style transition
        commands.entity(entity).insert(next_state);
    }
}


pub fn choice_input_stage_2<T: StateConstraint>(
    query: Query<(Entity, &MultipleChoice<T>, &ButtonStyle)>,
    choice_state: Res<State<T>>,
    mut commands: Commands
) {
    for (entity, choice, button) in &query {
        let this = *choice_state.get() == choice.choice;

        // Compute style transition but do not apply yet
        let next_state = match button {
            ButtonStyle::Idle => match this {
                true => ButtonStyle::Active,
                false => ButtonStyle::Idle,
            },
            ButtonStyle::Active => match this {
                true => ButtonStyle::Active,
                false => ButtonStyle::Idle,
            },
            ButtonStyle::Hover => match this {
                true => ButtonStyle::Active,
                false => ButtonStyle::Hover,
            },
        };

        // Apply style transition
        commands.entity(entity).insert(next_state);
    }
}
