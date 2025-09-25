use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::*;
pub use resources::*;


pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build (&self, app: &mut App) {
        app
            .add_observer(update_transform)
            .init_resource::<Grid>()
        ;
    }
}