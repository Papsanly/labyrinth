use crate::motion::{KeyboardMotion, MotionBundle};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default().mesh().ico(8).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#ff5471").unwrap(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
        MotionBundle::default(),
        KeyboardMotion,
        Ball,
    ));
}
