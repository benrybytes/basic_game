use bevy::prelude::*;

const CAMERA_Y: f32 = 50.0;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // spawn the camera entity to be facing the position 0,0,0
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20.0, CAMERA_Y, 0.0)
                // .rotate_y(90.0_f32.to_radians())
                .looking_at(Vec3::ZERO, Vec3::Z) // look at ground, from z
                
        ,MainCamera
    ));
}
