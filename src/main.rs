mod ball;
mod camera;
mod labyrinth;
mod lighting;
mod motion;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(motion::MotionPlugin)
        .add_plugins(ball::BallPlugin)
        .add_plugins(labyrinth::LabyrinthPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(lighting::LightingPlugin)
        .run();
}
