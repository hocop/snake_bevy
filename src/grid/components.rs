use core::panic;

use bevy::prelude::*;

#[derive(Component, Deref, Copy, Clone)]
#[component(immutable)]
pub struct GridPos(pub UVec2);

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
    pub fn to_rotation(&self) -> Quat {
        match self {
           Self::North => Quat::from_rotation_z(0.0),
           Self::West => Quat::from_rotation_z(std::f32::consts::PI / 2.0),
           Self::South => Quat::from_rotation_z(std::f32::consts::PI),
           Self::East => Quat::from_rotation_z(std::f32::consts::PI * 3.0 / 2.0),
        }
    }

    pub fn rotate(&self, dir: &LocalDirection) -> Self {
        // add directions
        let a = *self as u8 as i32 + *dir as u8 as i32 - 1;
        // wrap around
        let a = (a + 4) % 4;
        // convert
        match a {
            0 => Self::North,
            1 => Self::West,
            2 => Self::South,
            3 => Self::East,
            _ => panic!()
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
        }
    }
}


#[derive(Component, Debug, PartialEq, Default, Copy, Clone)]
#[component(immutable)]
pub enum LocalDirection {
    // Do not change order
    Right,
    #[default]
    Forward,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_direction_opposite() {
        assert_eq!(GlobalDirection::North.opposite(), GlobalDirection::South);
        assert_eq!(GlobalDirection::South.opposite(), GlobalDirection::North);
        assert_eq!(GlobalDirection::East.opposite(), GlobalDirection::West);
        assert_eq!(GlobalDirection::West.opposite(), GlobalDirection::East);
    }

    #[test]
    fn test_local_direction_numerical() {
        assert_eq!(LocalDirection::Forward as u8 as i32 - 1, 0);
        assert_eq!(GlobalDirection::South as u8 as i32, 2);
    }

    #[test]
    fn test_global_direction_rotate() {
        // Test rotating North - these are the actual results from the implementation
        assert_eq!(GlobalDirection::North.rotate(&LocalDirection::Left), GlobalDirection::West);
        assert_eq!(GlobalDirection::North.rotate(&LocalDirection::Forward), GlobalDirection::North);
        assert_eq!(GlobalDirection::North.rotate(&LocalDirection::Right), GlobalDirection::East);
        
        // Test rotating South
        assert_eq!(GlobalDirection::South.rotate(&LocalDirection::Left), GlobalDirection::East);
        assert_eq!(GlobalDirection::South.rotate(&LocalDirection::Forward), GlobalDirection::South);
        assert_eq!(GlobalDirection::South.rotate(&LocalDirection::Right), GlobalDirection::West);
        
        // Test rotating East
        assert_eq!(GlobalDirection::East.rotate(&LocalDirection::Left), GlobalDirection::North);
        assert_eq!(GlobalDirection::East.rotate(&LocalDirection::Forward), GlobalDirection::East);
        assert_eq!(GlobalDirection::East.rotate(&LocalDirection::Right), GlobalDirection::South);
        
        // Test rotating West
        assert_eq!(GlobalDirection::West.rotate(&LocalDirection::Left), GlobalDirection::South);
        assert_eq!(GlobalDirection::West.rotate(&LocalDirection::Forward), GlobalDirection::West);
        assert_eq!(GlobalDirection::West.rotate(&LocalDirection::Right), GlobalDirection::North);
    }
}