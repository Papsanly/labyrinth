use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;
use std::f32::consts::{FRAC_PI_2, SQRT_2};

#[derive(Component, Debug)]
pub struct MotionBody {
    pub velocity: Vec3,
    pub force: Vec3,
    pub mass: f32,
}

impl Default for MotionBody {
    fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec3::ZERO,
            force: Vec3::ZERO,
        }
    }
}

#[derive(Component, Debug)]
pub struct KeyboardMotion {
    pub force_magnitude: f32,
    pub friction: f32,
}

impl Default for KeyboardMotion {
    fn default() -> Self {
        Self {
            force_magnitude: 1.0,
            friction: 1.0,
        }
    }
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_motion, keyboard_motion).chain());
    }
}

fn update_motion(mut query: Query<(&mut Transform, &mut MotionBody)>, time: Res<Time>) {
    for (mut transform, mut body) in &mut query {
        let dt = time.delta_seconds();
        body.velocity = body.velocity + body.force / body.mass * dt;
        transform.translation += body.velocity * dt;
    }
}

fn keyboard_motion(
    mut motion_query: Query<(&mut MotionBody, &KeyboardMotion)>,
    camera_transform_query: Query<&Transform, With<ThirdPersonCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    use KeyCode::{KeyA, KeyD, KeyS, KeyW};

    let (
        mut body,
        &KeyboardMotion {
            force_magnitude,
            friction,
        },
    ) = motion_query.single_mut();

    let &Transform {
        rotation: camera_rotation,
        ..
    } = camera_transform_query.single();

    let camera_direction = camera_rotation * Vec3::NEG_Z;
    let forward = Vec3::new(camera_direction.x, 0.0, camera_direction.z).normalize();
    let right = Quat::from_rotation_y(-FRAC_PI_2) * forward;
    let mut force = Vec3::ZERO;

    if input.all_pressed([KeyW, KeyD]) {
        force = (forward + right) / SQRT_2;
    } else if input.all_pressed([KeyW, KeyA]) {
        force = (forward - right) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyD]) {
        force = (-forward + right) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyA]) {
        force = (-forward - right) / SQRT_2;
    } else if input.pressed(KeyW) {
        force = forward;
    } else if input.pressed(KeyS) {
        force = -forward;
    } else if input.pressed(KeyD) {
        force = right;
    } else if input.pressed(KeyA) {
        force = -right;
    }

    if input.just_released(KeyW) {
        force -= forward;
    } else if input.just_released(KeyS) {
        force += forward;
    } else if input.just_released(KeyD) {
        force -= right;
    } else if input.just_released(KeyA) {
        force += right;
    }

    body.force = force * force_magnitude - friction * body.velocity;
}
