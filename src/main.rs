#![feature(trait_alias)]
use bevy::prelude::*;

use crate::{actions::ActionsPlugin, app_state::AppState, buttons::ButtonsPlugin, food::{Food, FoodPlugin}, game_loop::GameLoopPlugin, grid::{Grid, GridPlugin}, rng::RngPlugin, snake::{spawn_snake, Body, SnakePlugin}, themes::ThemesPlugin, ui::FrontendPlugin};

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

        .add_systems(Startup, (
            setup_camera,
        ))
        .add_systems(OnEnter(AppState::Play), (
            move_camera_to_overview,
            setup_scene,
        ).chain())
        .add_systems(OnExit(AppState::Play), (
            despawn_scene,
        ).chain())
        .run();
}


const PIXELS_PER_METER: f32 = 16.0;

pub fn setup_scene(
    grid: Res<Grid>,
    mut commands: Commands,
) {
    spawn_snake(grid.size / 2, &grid, &mut commands);
}

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d {},
        Projection::Orthographic(OrthographicProjection {
            near: -1000.,
            scale: 1. / PIXELS_PER_METER,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn move_camera_to_overview(
    mut camera_pos: Query<&mut Transform, With<Camera>>,
    mut camera_proj: Query<&mut Projection, With<Camera>>,
    window_query: Query<&Window>,
    grid: Res<Grid>
) -> Result<(), BevyError> {
    // Find AABB
    let (min, max) = grid.aabb();

    // Position camera
    let mid = min.midpoint(max);
    camera_pos.single_mut()?.translation = Vec3::new(mid.x, mid.y, 0.0);

    // Set scale
    let Vec2 { x: w, y: h } = max - min;
    let window = window_query.single()?;
    let scale = f32::max(
        h / window.height(),
        w / window.width()
    );
    change_scale(
        &mut camera_proj,
        |_| scale
    )?;

    Ok(())
}

pub fn change_scale<S>(
    camera_proj: &mut Query<&mut Projection, With<Camera>>,
    new_scale: S
) -> Result<(), BevyError>
where S: Fn(f32) -> f32,
{
    match camera_proj.single()? {
        Projection::Orthographic(projection) => {
            *camera_proj.single_mut()? = Projection::Orthographic(OrthographicProjection {
                scale: new_scale(projection.scale),
                ..*projection
            });
        },
        _ => { todo!() }
    };
    Ok(())
}



// Implement despawn_scene system
pub fn despawn_scene(
    mut commands: Commands,
    snake: Query<Entity, With<Body>>,
    food: Query<Entity, With<Food>>,
) {
    for entity in snake.iter().chain(food.iter()) {
        commands.entity(entity).try_despawn();
    }
}
