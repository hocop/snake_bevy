use bevy::prelude::*;

use crate::{food::Food, grid::Grid, snake::*};


pub fn setup_scene(
    grid: Res<Grid>,
    mut commands: Commands,
) {
    spawn_snake(grid.size / 2, &grid, &mut commands);
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
