use std::time::Duration;

use bevy::prelude::*;

mod components;
mod systems;
mod factory;

use systems::*;
pub use factory::*;


pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
            .add_observer(add_sprite)
            .add_systems(FixedUpdate, snake_step)
            .add_systems(Update, steer_snake)
        ;
    }
}
