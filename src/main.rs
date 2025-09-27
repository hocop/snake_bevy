#![feature(trait_alias)]
use bevy::prelude::*;

use crate::{actions::ActionsPlugin, app_state::AppState, buttons::ButtonsPlugin, food::FoodPlugin, game_loop::GameLoopPlugin, grid::GridPlugin, rng::RngPlugin, snake::SnakePlugin, themes::ThemesPlugin, ui::FrontendPlugin};

mod ui;
mod buttons;
mod app_state;
mod actions;
mod grid;
mod themes;
mod snake;
mod food;
mod game_loop;
mod rng;
mod camera;
mod scene;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()

        .add_plugins(FrontendPlugin)
        .add_plugins(ThemesPlugin)
        .add_plugins(ActionsPlugin)
        .add_plugins(ButtonsPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(GameLoopPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(RngPlugin)

        .run();
}
