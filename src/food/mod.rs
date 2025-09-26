use bevy::prelude::*;

use crate::{grid::{components::GridPos, Grid}, rng::RandomSource};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, spawn_food)
            .add_observer(add_sprite_food)
        ;
    }
}

pub fn spawn_food(
    mut commands: Commands,
    query: Query<&Food>,
    mut random_source: ResMut<RandomSource>,
    grid: Res<Grid>,
) {
    if query.is_empty() {
        commands.spawn((
            Food {},
            grid.random_unoccupied_pos(&mut random_source),
        ));
    }
}

#[derive(Component)]
pub struct Food {}


pub fn add_sprite_food(
    trigger: Trigger<OnAdd, Food>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(trigger.target()).insert(InheritedVisibility::default());

    // Create a marker entity with a circle shape and a color
    let shape = meshes.add(Circle::new(0.45));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        ChildOf(trigger.target()),
    ));
}
