use bevy::prelude::*;

mod components;
mod systems;
mod factory;

pub use systems::*;
pub use factory::*;
pub use components::*;


pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app
            .add_observer(add_sprite_snake)
            .add_systems(Update, snake_process_controls)
        ;
    }
}
