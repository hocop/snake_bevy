use bevy::prelude::*;

use crate::grid::Grid;

const PIXELS_PER_METER: f32 = 16.0;


pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d {},
        Projection::Orthographic(OrthographicProjection {
            near: -1000.,
            scale: 1. / PIXELS_PER_METER,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn move_camera_to_overview(
    mut camera_pos: Query<&mut Transform, With<Camera>>,
    mut camera_proj: Query<&mut Projection, With<Camera>>,
    window_query: Query<&Window>,
    grid: Res<Grid>
) -> Result<(), BevyError> {
    // Find AABB
    let (min, max) = grid.aabb();

    // Position camera
    let mid = min.midpoint(max);
    camera_pos.single_mut()?.translation = Vec3::new(mid.x, mid.y, 0.0);

    // Set scale
    let Vec2 { x: w, y: h } = max - min;
    let window = window_query.single()?;
    let scale = f32::max(
        h / window.height(),
        w / window.width()
    );
    change_scale(
        &mut camera_proj,
        |_| scale
    )?;

    Ok(())
}

pub fn change_scale<S>(
    camera_proj: &mut Query<&mut Projection, With<Camera>>,
    new_scale: S
) -> Result<(), BevyError>
where S: Fn(f32) -> f32,
{
    match camera_proj.single()? {
        Projection::Orthographic(projection) => {
            *camera_proj.single_mut()? = Projection::Orthographic(OrthographicProjection {
                scale: new_scale(projection.scale),
                ..*projection
            });
        },
        _ => { todo!() }
    };
    Ok(())
}