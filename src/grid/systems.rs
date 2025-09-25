use bevy::prelude::*;

use crate::grid::{components::GridPos, Grid};


pub fn update_transform(
    trigger: Trigger<OnInsert, GridPos>,
    pos_query: Query<&GridPos>,
    mut commands: Commands
) {
    let entity = trigger.target();
    let pos = pos_query.get(entity).unwrap();
    commands.entity(entity).insert(pos.to_transform());
}


pub fn remove_from_index(
    trigger: Trigger<OnReplace, GridPos>,
    pos_query: Query<&GridPos>,
    mut grid: ResMut<Grid>
) {
    let entity = trigger.target();
    let pos = pos_query.get(entity).unwrap();
    grid.set(pos, None);
}


pub fn add_to_index(
    trigger: Trigger<OnInsert, GridPos>,
    pos_query: Query<&GridPos>,
    mut grid: ResMut<Grid>
) {
    let entity = trigger.target();
    let pos = pos_query.get(entity).unwrap();
    grid.set(pos, Some(entity));
}
