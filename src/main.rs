mod ball;
mod camera;
mod labyrinth;
mod lighting;
mod motion;

use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(motion::MotionPlugin)
        .add_plugins(ball::BallPlugin)
        .add_plugins(labyrinth::LabyrinthPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(lighting::LightingPlugin)
        .run();
}
