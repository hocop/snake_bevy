use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::HUDAction, grid::{components::*, Grid}, snake::components::*};

pub fn snake_step(
    heads: Query<(Entity, &Head, &Body, &GridPos, &GlobalDirection, &LocalDirection)>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    let size = grid.size;

    for (entity, head, body, pos, dir, ldir) in &heads {
        // Compute new head position
        let dir = &dir.rotate(ldir);
        let new_pos = pos.shift(dir, size);

        // Do a cyclic shift
        // Old head stops being head, steering is reset
        commands.entity(entity)
            .remove::<Head>()
            .insert(LocalDirection::default());

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


pub fn snake_eat(
    heads: Query<(Entity, &Head, &Body, &GridPos, &GlobalDirection, &LocalDirection)>,
    grid: Res<Grid>,
    mut commands: Commands
) {
    let size = grid.size;

    for (entity, head, body, pos, dir, ldir) in &heads {
        // Compute new head position
        let dir = &dir.rotate(ldir);
        let new_pos = pos.shift(dir, size);

        // Do a cyclic shift
        // Old head stops being head, steering is reset
        commands.entity(entity)
            .remove::<Head>()
            .insert(LocalDirection::default());

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


pub fn steer_snake(
    action_state: Res<ActionState<HUDAction>>,
    mut heads: Query<&mut LocalDirection, With<Head>>,
) {
    let new_dir = if action_state.just_pressed(&HUDAction::GoLeft) {
        Some(LocalDirection::Left)
    } else if action_state.just_pressed(&HUDAction::GoRight) {
        Some(LocalDirection::Right)
    } else { None };
    if let Some(new_dir) = new_dir {
        for mut dir in heads.iter_mut() {
            *dir = new_dir;
        }
    }
}


pub fn add_sprite_snake(
    trigger: Trigger<OnAdd, Body>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.entity(trigger.target()).insert(InheritedVisibility::default());

    // Create a marker entity with a circle shape and a color
    let shape = meshes.add(Rectangle::new(0.9, 0.9));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::BLACK)),
        ChildOf(trigger.target()),
    ));
}
