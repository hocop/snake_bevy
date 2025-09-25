use bevy::prelude::*;

use crate::{grid::{components::*, Grid}, snake::components::*};


pub fn grow_head(
    heads: Query<(Entity, &Head, &GridPos, &GlobalDirection)>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    let size = grid.size;

    for (entity, head, pos, dir) in &heads {
        let new_pos = pos.shift(dir, size);
        let new_body = commands.spawn((
            Body { prev: entity, next: head.next },
            *pos,
            *dir,
        )).id();
        commands.entity(entity).insert((Head { next: new_body }, new_pos));
    }
}

pub fn shrink_tail(
    tails: Query<(Entity, &Tail)>,
    bodys: Query<&Body>,
    mut commands: Commands
) {
    for (entity, tail) in &tails {
        let prev_body = bodys.get(tail.prev).unwrap();
        commands.entity(tail.prev).insert((
            Tail { prev: prev_body.prev },
        ));
        commands.entity(entity).despawn();
    }
}
