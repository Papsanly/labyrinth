mod ball;
mod camera;
mod labyrinth;
mod lighting;
mod motion;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            motion::MotionPlugin,
            ball::BallPlugin,
            labyrinth::LabyrinthPlugin,
            camera::CameraPlugin,
            lighting::LightingPlugin,
        ))
        .run();
}
