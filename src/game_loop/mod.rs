use std::time::Duration;

use bevy::prelude::*;

use crate::{app_state::AppState, food::*, snake::*};

pub struct GameLoopPlugin;



impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
            .add_systems(Startup, stop_time)
            .add_systems(OnEnter(AppState::Play), (
                spawn_food,
                start_time,
            ).chain())
            .add_systems(OnExit(AppState::Play), (
                stop_time,
            ).chain())
            .add_systems(FixedUpdate, (
                snake_steer,
                snake_eat,
                snake_step,
                spawn_food,
            ).chain())
        ;
    }
}

pub fn start_time(
    mut time: ResMut<Time<Virtual>>
) {
    time.unpause();
}

pub fn stop_time(
    mut time: ResMut<Time<Virtual>>
) {
    time.pause();
}
