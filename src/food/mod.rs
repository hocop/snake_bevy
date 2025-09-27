use bevy::prelude::*;

use crate::{grid::Grid, rng::RandomSource};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SnakeStats>()
            .add_systems(FixedUpdate, spawn_food)
            .add_observer(add_sprite_food)
            .add_observer(increase_score)
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
        if let Some(pos) = grid.random_unoccupied_pos(&mut random_source) {
            commands.spawn((
                Food {},
                pos,
            ));
        }
    }
}

pub fn increase_score(
    _trigger: Trigger<OnRemove, Food>,
    mut stats: ResMut<SnakeStats>
) {
    stats.eaten += 1;
}


#[derive(Component)]
pub struct Food {}

#[derive(Resource, Default)]
pub struct SnakeStats {
    pub eaten: u32,
}

pub fn add_sprite_food(
    trigger: Trigger<OnAdd, Food>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(trigger.target()).insert(InheritedVisibility::default());

    // Create a marker entity with a circle shape and a color
    let shape = meshes.add(Circle::new(0.5));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
        ChildOf(trigger.target()),
    ));
}
