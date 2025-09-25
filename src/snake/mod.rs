use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod factory;

use systems::*;
pub use resources::*;


pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {

    }
}