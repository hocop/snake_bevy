use bevy::prelude::*;

use crate::{grid::{components::{GlobalDirection, GridPos}, Grid}, snake::components::*};


pub fn spawn_snake(
    origin: UVec2,
    grid: &Res<Grid>,
    commands: &mut Commands
) {
    let dir = GlobalDirection::default();
    let [head, body, tail] = (0..3)
        .scan(GridPos(origin), |pos, _| {
            let entity = commands.spawn((*pos, dir)).id();
            *pos = pos.shift(&dir.opposite(), grid.size);
            Some(entity)
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // snake is a cyclical linked list
    commands.entity(head).insert((Head::default(), Body {prev: tail}));
    commands.entity(body).insert(Body {prev: head});
    commands.entity(tail).insert(Body {prev: body});
}
