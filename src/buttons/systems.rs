use bevy::prelude::*;

use crate::{buttons::{components::ButtonStyle, resources::AnyButtonHovered}, themes::UITheme};


pub fn button_style(
    mut interaction_query: Query<(&ButtonStyle, &mut BorderColor, &mut BackgroundColor), Changed<ButtonStyle>>,
    theme: Res<UITheme>,
) {
    for (button, mut border_color, mut background_color) in &mut interaction_query {
        match *button {
            ButtonStyle::Idle => {
                border_color.0 = theme.button_idle.to_color();
                background_color.0 = theme.button_idle.to_color();
            },
            ButtonStyle::Active => {
                border_color.0 = theme.button_active.to_color();
                background_color.0 = theme.button_active.to_color();
            },
            ButtonStyle::Hover => {
                border_color.0 = theme.button_hover.to_color();
                background_color.0 = theme.button_hover.to_color();
            },
        }
    }
}


pub fn update_hovering_state(
    interaction_query: Query<&Interaction, (With<ButtonStyle>, Changed<Interaction>)>,
    mut any_button_hovered: ResMut<AnyButtonHovered>,
) {
    for interaction in &interaction_query {
        // Inform other systems of any button being hovered
        any_button_hovered.0 = match interaction {
            Interaction::Pressed => true,
            Interaction::Hovered => true,
            Interaction::None => false,
        };
    }
}
