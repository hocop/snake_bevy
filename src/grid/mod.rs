use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;
pub use resources::*;


pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_observer(update_transform)
            .add_observer(remove_from_index)
            .add_observer(add_to_index)
            .init_resource::<Grid>()
        ;
    }
}
