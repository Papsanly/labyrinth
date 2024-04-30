use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, SQRT_2};

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Friction(pub f32);

impl Default for Friction {
    fn default() -> Self {
        Self(1.0)
    }
}

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
    mass: Mass,
    friction: Friction,
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_motion, keyboard_motion).chain());
    }
}

fn update_motion(
    mut query: Query<(&mut Transform, &mut Velocity, &Force, &Mass, &Friction)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, force, mass, friction) in &mut query {
        let dt = time.delta_seconds();
        let v = **velocity;
        **velocity += (**force - **friction * v) / **mass * dt;
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
