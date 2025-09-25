use bevy::{prelude::*, ui::widget::NodeImageMode};

use crate::{actions::HUDAction, buttons::components::{SimpleButton, Toggle}};


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


fn square_button_with_icon(icon: Handle<Image>) -> impl Bundle {
    (
        Node {
            width: Val::Auto,
            height: Val::Percent(80.),
            aspect_ratio: Some(1.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Percent(20.0)),
        ImageNode::new(icon)
        .with_mode(NodeImageMode::Stretch)
    )
}


pub fn hud_simple(icon: Handle<Image>, action: HUDAction) -> impl Bundle {
    (
        square_button_with_icon(icon),
        SimpleButton::new(action),
    )
}


pub fn hud_toggle(icon: Handle<Image>, toggle: Toggle) -> impl Bundle {
    (
        square_button_with_icon(icon),
        toggle,
    )
}
