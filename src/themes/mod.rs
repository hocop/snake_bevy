use bevy::prelude::*;
use color_definition::ColorDefinition;
use serde::{Deserialize, Serialize};
use std::fs;

mod color_definition;

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct UITheme {
    pub ground: ColorDefinition,
    pub button_idle: ColorDefinition,
    pub button_hover: ColorDefinition,
    pub button_active: ColorDefinition,
}

pub struct ThemesPlugin;

impl Plugin for ThemesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(load_ui_theme())
            .add_systems(Startup, set_background_color)
        ;
    }
}

fn set_background_color(
    mut bg_color: ResMut<ClearColor>,
    ui_theme: Res<UITheme>,
) {
    bg_color.0 = ui_theme.ground.to_color();
}

fn load_ui_theme() -> UITheme {
    // let path = "assets/themes/ui_toy.json";
    let path = "assets/themes/ui_pale.json";
    let theme_str = fs::read_to_string(path).expect("Failed to read theme file");
    let theme: UITheme = serde_json::from_str(&theme_str).expect("Failed to parse theme JSON");

    theme
}
