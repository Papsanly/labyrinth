use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, SQRT_2};

const FRICTION: f32 = 0.5;
const ACCELERATION: f32 = 2.0;

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Acceleration(pub Vec3);

#[derive(Component)]
pub struct KeyboardMotion;

#[derive(Bundle, Default)]
pub struct MotionBundle {
    velocity: Velocity,
    acceleration: Acceleration,
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_motion, keyboard_motion).chain());
    }
}

fn update_motion(
    mut query: Query<(&mut Transform, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, acceleration) in &mut query {
        let dt = time.delta_seconds();
        **velocity = **velocity + (**acceleration - FRICTION * **velocity) * dt;
        transform.translation += **velocity * dt;
    }
}

fn keyboard_motion(
    mut acceleration_query: Query<&mut Acceleration, With<KeyboardMotion>>,
    camera_transform_query: Query<&Transform, With<Camera3d>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    use KeyCode::{KeyA, KeyD, KeyS, KeyW};

    let mut acceleration = acceleration_query.single_mut();
    let &Transform {
        rotation: camera_rotation,
        ..
    } = camera_transform_query.single();

    let camera_direction = camera_rotation * Vec3::NEG_Z;
    let camera_direction = Vec3::new(camera_direction.x, 0.0, camera_direction.z).normalize();
    let forward = camera_direction * ACCELERATION;
    let left = Quat::from_rotation_y(FRAC_PI_2) * forward * ACCELERATION;

    if input.all_pressed([KeyW, KeyD]) {
        **acceleration = (forward - left) / SQRT_2;
    } else if input.all_pressed([KeyW, KeyA]) {
        **acceleration = (forward + left) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyD]) {
        **acceleration = (-forward - left) / SQRT_2;
    } else if input.all_pressed([KeyS, KeyA]) {
        **acceleration = (-forward + left) / SQRT_2;
    } else if input.pressed(KeyW) {
        **acceleration = forward;
    } else if input.pressed(KeyS) {
        **acceleration = -forward;
    } else if input.pressed(KeyD) {
        **acceleration = -left;
    } else if input.pressed(KeyA) {
        **acceleration = left;
    }

    if input.just_released(KeyW) {
        **acceleration -= forward;
    } else if input.just_released(KeyS) {
        **acceleration += forward;
    } else if input.just_released(KeyD) {
        **acceleration += left;
    } else if input.just_released(KeyA) {
        **acceleration -= left;
    }
}
