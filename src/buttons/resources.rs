use bevy::prelude::*;


#[derive(Resource)]
pub struct AnyButtonHovered(pub bool);

impl Default for AnyButtonHovered {
    fn default() -> Self {
        Self(false)
    }
}
