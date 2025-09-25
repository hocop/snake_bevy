use bevy::prelude::*;

use crate::grid::components::GridPos;


pub fn update_transform(
    trigger: Trigger<OnInsert, GridPos>,
    pos_query: Query<&GridPos>,
    mut commands: Commands
) {
    let entity = trigger.target();
    let pos = pos_query.get(entity).unwrap();
    commands.entity(entity).insert(pos.to_transform());
}
