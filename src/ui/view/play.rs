use bevy::prelude::*;

use crate::{actions::HUDAction, ui::elements::menu_button};


pub fn build_play() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Stretch,
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Start,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    ..default()
                },
                children![
                    menu_button("Menu", HUDAction::ToMenu),
                ]
            ),
            (
                Node {
                    width: Val::Auto,
                    height: Val::Percent(100.),
                    aspect_ratio: Some(1.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                BorderColor(Color::BLACK),
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Start,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    ..default()
                },
            ),
        ]
    )
}
