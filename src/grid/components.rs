use core::panic;

use bevy::prelude::*;

#[derive(Component, Deref, Copy, Clone)]
#[component(immutable)]
pub struct GridPos(pub UVec2);

// fn wrap_add(a: u32, b: i32, m: u32) -> u32 {
//     match b {
//         (0..) => (a + b as u32) % m,
//         (..0) => (a + m - b as u32) % m
//     }
// }

fn wrap_inc(a: u32, m: u32) -> u32 {
    (a + 1) % m
}

fn wrap_dec(a: u32, m: u32) -> u32 {
    (a + m - 1) % m
}

impl GridPos {
    pub fn to_transform(&self) -> Transform {
        Transform::from_translation(Vec3::new(self.x as f32, self.y as f32, 0.0))
    }

    pub fn shift(&self, dir: &GlobalDirection, size: UVec2) -> Self {
        let new_pos = match dir {
            GlobalDirection::North => UVec2::new(self.x, wrap_inc(self.y, size.y)),
            GlobalDirection::West => UVec2::new(wrap_dec(self.x, size.x), self.y),
            GlobalDirection::South => UVec2::new(self.x, wrap_dec(self.y, size.y)),
            GlobalDirection::East => UVec2::new(wrap_inc(self.x, size.x), self.y),
        };
        GridPos(new_pos)
    }
}

#[derive(Component, Debug, PartialEq, Default, Copy, Clone)]
pub enum GlobalDirection {
    #[default]
    North,
    West,
    South,
    East,
}

impl GlobalDirection {
    pub fn rotate(&self, dir: &LocalDirection) -> Self {
        let a = *self as u8 as i32 + *dir as u8 as i32 - 1;
        // wrap
        let a = if a > 0 {a} else {a + 4};
        match a {
            0 => Self::North,
            1 => Self::West,
            2 => Self::South,
            3 => Self::East,
            _ => panic!()
        }
    }
}


#[derive(Component, Debug, PartialEq, Default, Copy, Clone)]
#[component(immutable)]
pub enum LocalDirection {
    Left,
    #[default]
    Forward,
    Right,
}