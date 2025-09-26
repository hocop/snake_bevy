use bevy::prelude::*;
use rand::{distr::Uniform, Rng};

use crate::{grid::components::GridPos, rng::RandomSource};

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


impl Grid {
    pub fn get(&self, pos: &UVec2) -> Option<Entity> {
        self.index[self.pos_to_i(pos)]
    }

    pub fn set(&mut self, pos: &UVec2, value: Option<Entity>) {
        let i = self.pos_to_i(pos);
        self.index[i] = value;
    }

    pub fn aabb(&self) -> (Vec2, Vec2) {
        let min = Vec2::new(-0.5, -0.5);
        let max = Vec2::new(self.size.x as f32 - 0.5, self.size.y as f32 - 0.5);
        (min, max)
    }

    pub fn random_unoccupied_pos(&self, rng: &mut RandomSource) -> GridPos {
        let mut unoccupied_positions = Vec::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = UVec2::new(x, y);
                if self.get(&pos).is_none() {
                    unoccupied_positions.push(pos);
                }
            }
        }
        if unoccupied_positions.is_empty() {
            panic!("No unoccupied positions found");
        }
        let i = rng.0.sample(Uniform::new(0, unoccupied_positions.len()).unwrap());
        GridPos(unoccupied_positions[i])
    }

    fn pos_to_i(&self, pos: &UVec2) -> usize {
        pos.x as usize + self.size.x as usize * pos.y as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn i_to_pos(grid: &Grid, i: usize) -> UVec2 {
        UVec2::new((i % grid.size.y as usize) as u32, (i / grid.size.y as usize) as u32)
    }

    #[test]
    fn test_index() {
        let grid = Grid::default();

        for i in 0..256 {
            assert_eq!(i, grid.pos_to_i(&i_to_pos(&grid, i)));
        }
    }
}
