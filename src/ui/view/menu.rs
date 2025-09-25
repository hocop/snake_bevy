use bevy::prelude::*;

use crate::{actions::HUDAction, ui::elements::menu_button};


pub fn build_menu() -> impl Bundle {
    // Main column
    (
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            // Play button
            menu_button("Play", HUDAction::EnterLevel),
            // Settings button
            menu_button("Settings", HUDAction::Placeholder),
            // Exit button
            menu_button("Exit", HUDAction::ExitGame),
        ]
    )
}
