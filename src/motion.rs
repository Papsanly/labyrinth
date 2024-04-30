use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, SQRT_2};

const FRICTION: f32 = 0.5;
const MASS: f32 = 0.5;

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Force(pub Vec3);

#[derive(Component)]
pub struct KeyboardMotion;

#[derive(Bundle, Default)]
pub struct MotionBundle {
    velocity: Velocity,
    force: Force,
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_motion, keyboard_motion).chain());
    }
}

fn update_motion(mut query: Query<(&mut Transform, &mut Velocity, &Force)>, time: Res<Time>) {
    for (mut transform, mut velocity, force) in &mut query {
        let dt = time.delta_seconds();
        let v = **velocity;
        **velocity += (**force - FRICTION * v) / MASS * dt;
        transform.translation += v * dt;
    }
}

fn keyboard_motion(
    mut force_query: Query<&mut Force, With<KeyboardMotion>>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    use KeyCode::{KeyA, KeyD, KeyS, KeyW};

    let mut force = force_query.single_mut();
    let &Transform {
        rotation: camera_rotation,
        ..
    } = camera_transform_query.single();

    let camera_direction = camera_rotation * Vec3::NEG_Z;
    let forward = Vec3::new(camera_direction.x, 0.0, camera_direction.z).normalize();
    let right = Quat::from_rotation_y(-FRAC_PI_2) * forward;

    if input.all_pressed([KeyW, KeyD]) {
        **force = (forward + right) / SQRT_2;
    } else if input.all_pressed([KeyW, KeyA]) {
        **force = (forward - right) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyD]) {
        **force = (-forward + right) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyA]) {
        **force = (-forward - right) / SQRT_2;
    } else if input.pressed(KeyW) {
        **force = forward;
    } else if input.pressed(KeyS) {
        **force = -forward;
    } else if input.pressed(KeyD) {
        **force = right;
    } else if input.pressed(KeyA) {
        **force = -right;
    }

    if input.just_released(KeyW) {
        **force -= forward;
    } else if input.just_released(KeyS) {
        **force += forward;
    } else if input.just_released(KeyD) {
        **force -= right;
    } else if input.just_released(KeyA) {
        **force += right;
    }
}
