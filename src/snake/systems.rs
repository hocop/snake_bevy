use bevy::prelude::*;

use crate::{grid::{components::*, Grid}, snake::components::*};

pub fn snake_step(
    heads: Query<(Entity, &Head, &Body, &GridPos, &GlobalDirection)>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    let size = grid.size;

    for (entity, head, body, pos, dir) in &heads {
        // Compute new head position
        let new_pos = pos.shift(dir, size);

        // Do a cyclic shift
        // Old head stops being head
        commands.entity(entity).remove::<Head>();

        if head.has_eaten {
            // New head appears
            let new_head = commands.spawn((
                Head::default(),
                Body {prev: body.prev},
                new_pos,
                *dir
            )).id();
            // Old head starts pointing at the new head
            commands.entity(entity).insert(Body {prev: new_head});
        } else {
            // Tail becomes new head
            commands.entity(body.prev).insert((Head::default(), new_pos, *dir));
        }
    }
}


pub fn add_sprite(
    trigger: Trigger<OnInsert, Body>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(trigger.target()).insert(InheritedVisibility::default());

    // Create a marker entity with a circle shape and a color
    let shape = meshes.add(Circle::new(0.5));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::BLACK)),
        ChildOf(trigger.target()),
    ));
}
