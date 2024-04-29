use bevy::prelude::*;

const MAX_SPEED: f32 = 4.0;
const FRICTION: f32 = 0.9;

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec3);

#[derive(Bundle, Default)]
pub struct MotionBundle {
    velocity: Velocity,
    acceleration: Acceleration,
}

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(mut query: Query<(&mut Transform, &mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut transform, mut velocity, acceleration) in &mut query {
        let initial_velocity = velocity.0;
        let dt = time.delta_seconds();

        if velocity.0.length() > MAX_SPEED {
            velocity.0 = velocity.0.normalize() * MAX_SPEED;
        } else {
            velocity.0 += acceleration.0 * dt;
        }

        if acceleration.0.length() == 0.0 {
            velocity.0 -= initial_velocity * FRICTION * dt;
        }

        transform.translation += velocity.0 * dt;
    }
}
