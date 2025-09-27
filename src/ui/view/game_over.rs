use bevy::prelude::*;

use crate::{actions::HUDAction, ui::elements::menu_button};


pub fn build_game_over() -> impl Bundle {
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
            Text("Game Over!".into()),
            // Menu button
            menu_button("Menu", HUDAction::ToMenu),
        ]
    )
}
