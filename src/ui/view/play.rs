use bevy::prelude::*;

use crate::{actions::HUDAction, ui::elements::menu_button};


pub fn build_play() -> impl Bundle {
    // Main column
    (
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            align_self: AlignSelf::Start,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..default()
        },
        children![
            menu_button("Menu", HUDAction::ToMenu),
        ]
    )
}
