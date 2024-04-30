use crate::motion::{Acceleration, MotionBundle};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use std::f32::consts::{FRAC_PI_2, SQRT_2};

const ACCELERATION: f32 = 2.0;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, keyboard_motion);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default().mesh().ico(7).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#ff5471").unwrap(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
        MotionBundle::default(),
        Ball,
    ));
}

fn keyboard_motion(
    mut acceleration_query: Query<&mut Acceleration, With<Ball>>,
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
