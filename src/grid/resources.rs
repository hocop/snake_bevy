use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub size: UVec2,
    pub index: Vec<Option<Entity>>,
}


impl Default for Grid {
    fn default() -> Self {
        let size = UVec2::new(16, 16);
        let index = vec![None; (size.x * size.y) as usize];
        Self { size, index }
    }
}
