use bevy::prelude::*;

mod components;
mod systems;
mod factory;

use systems::*;


pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app
        .add_systems(
            Update,
            snake_step
        )
        ;
    }
}
