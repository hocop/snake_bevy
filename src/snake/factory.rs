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

    commands.entity(head).insert(Head {next: body});
    commands.entity(body).insert((Body {prev: head, next: body}, ChildOf(head)));
    commands.entity(tail).insert((Tail {prev: body}, ChildOf(head)));
}
