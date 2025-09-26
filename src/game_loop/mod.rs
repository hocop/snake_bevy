use std::time::Duration;

use bevy::prelude::*;

use crate::{food::*, snake::*};

pub struct GameLoopPlugin;



impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
            .add_systems(FixedUpdate, (
                snake_step,
                spawn_food,
            ).chain())
        ;
    }
}
