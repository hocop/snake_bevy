use bevy::prelude::*;

use crate::{actions::HUDAction, buttons::components::SimpleButton};


pub fn menu_button(text: &str, action: HUDAction) -> impl Bundle {
    (
        SimpleButton::new(action),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Percent(20.0)),
        children![
            (
                Text(text.into()),
                TextColor(Color::BLACK),
            )
        ]
    )
}
