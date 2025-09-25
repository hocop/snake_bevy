use bevy::prelude::*;

use crate::snake::components::*;


pub fn grow_head(
    heads: Query<&Head>,
    bodys: Query<&Body>,
    mut commands: Commands
) {
    for head in &heads {
        // head.next
    }
}

pub fn shrink_tail(
    tails: Query<&Tail>,
    bodys: Query<&Body>,
    mut commands: Commands
) {
    
}
