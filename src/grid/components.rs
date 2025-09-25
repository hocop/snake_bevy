use bevy::prelude::*;

#[derive(Component, Deref)]
#[component(immutable)]
pub struct GridPos(pub UVec2);

impl GridPos {
    pub fn to_transform(&self) -> Transform {
        Transform::from_translation(Vec3::new(self.x as f32, self.y as f32, 0.0))
    }
}