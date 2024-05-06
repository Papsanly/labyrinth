use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn, cursor_grab))
            .add_systems(Update, (orbit_camera, follow_orbit_target).chain());
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub sensitivity: Vec2,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            radius: 5.0,
            sensitivity: Vec2::new(1.0, 1.0),
        }
    }
}

#[derive(Component)]
pub struct OrbitCameraTarget;

fn spawn(mut commands: Commands) {
    commands.spawn((OrbitCamera::default(), Camera3dBundle::default()));
}

fn orbit_camera(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut orbit_q: Query<(&mut OrbitCamera, &mut Transform), Without<OrbitCameraTarget>>,
    mut ev_motion: EventReader<MouseMotion>,
) {
    let window = window_q.single();
    let (orbit, mut transform) = orbit_q.single_mut();

    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.read() {
        rotation_move += ev.delta;
    }

    if rotation_move.length_squared() > 0.0 {
        let delta_x = rotation_move.x / window.width() * PI * 2.0 * orbit.sensitivity.x;
        let delta_y = rotation_move.y / window.height() * PI * orbit.sensitivity.y;

        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);

        transform.rotation = yaw * transform.rotation;

        let new_rotation = transform.rotation * pitch;
        let up_vector = new_rotation * Vec3::Y;
        let camera_vector = new_rotation * Vec3::NEG_Z;
        if up_vector.y > 0.0 && camera_vector.y < 0.0 {
            transform.rotation = new_rotation;
        }

        transform.translation = transform.rotation * Vec3::new(0.0, 0.0, orbit.radius);
    }

    ev_motion.clear();
}

fn follow_orbit_target(
    mut orbit_q: Query<(&mut Transform, &OrbitCamera), Without<OrbitCameraTarget>>,
    orbit_target_q: Query<&Transform, With<OrbitCameraTarget>>,
) {
    let target_transform = orbit_target_q.single();
    let (mut camera_transform, camera) = orbit_q.single_mut();
    let desired_translation = camera_transform.rotation * Vec3::new(0.0, 0.0, camera.radius);
    camera_transform.translation = desired_translation + target_transform.translation;
}

fn cursor_grab(mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window_q.single_mut();
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}
