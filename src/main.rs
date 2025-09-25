#![feature(trait_alias)]
use bevy::prelude::*;

use crate::{actions::ActionsPlugin, app_state::AppState, buttons::ButtonsPlugin, grid::GridPlugin, snake::SnakePlugin, themes::ThemesPlugin, ui::FrontendPlugin};

mod ui;
mod buttons;
mod app_state;
mod actions;
mod grid;
mod themes;
mod snake;


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

        .add_systems(Startup, (
                setup_camera,  // Setup the camera after
            ))
        .run();
}


const PIXELS_PER_METER: f32 = 16.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d {},
        Projection::Orthographic(OrthographicProjection {
            near: -1000.,
            scale: 1. / PIXELS_PER_METER,
            ..OrthographicProjection::default_2d()
        }),
    ));
}