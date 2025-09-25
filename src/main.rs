#![feature(trait_alias)]
use bevy::prelude::*;

use crate::{actions::ActionsPlugin, app_state::AppState, buttons::ButtonsPlugin, themes::ThemesPlugin, ui::FrontendPlugin};

mod ui;
mod buttons;
mod app_state;
mod actions;
mod grid;
mod themes;

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()

        .add_plugins(FrontendPlugin)
        .add_plugins(ThemesPlugin)
        .add_plugins(ActionsPlugin)
        .add_plugins(ButtonsPlugin)
        .run();
}
