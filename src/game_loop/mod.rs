use std::time::Duration;

use bevy::prelude::*;

use crate::{app_state::AppState, camera::*, food::*, scene::*, snake::*};

pub struct GameLoopPlugin;



impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(500)))

            .add_systems(Startup, (
                stop_time,
                setup_camera,
            ))

            .add_systems(OnEnter(AppState::Play), (
                move_camera_to_overview,
                setup_scene,
                spawn_food,
                start_time,
            ))

            .add_systems(FixedUpdate, (
                snake_steer,
                snake_eat,
                snake_step,
                spawn_food,
            ).chain())

            .add_systems(OnExit(AppState::Play),
                stop_time,
            )
            .add_systems(OnEnter(AppState::Menu),
                despawn_scene,
            )
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
