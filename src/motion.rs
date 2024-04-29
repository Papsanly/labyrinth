use bevy::prelude::*;

const MAX_SPEED: f32 = 4.0;
const FRICTION: f32 = 0.9;

#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Debug, Deref, DerefMut)]
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
        let initial_velocity = **velocity;
        let dt = time.delta_seconds();

        if velocity.length() > MAX_SPEED {
            **velocity = velocity.normalize() * MAX_SPEED;
        } else {
            **velocity += **acceleration * dt;
        }

        if acceleration.length() == 0.0 {
            **velocity -= initial_velocity * FRICTION * dt;
        }

        transform.translation += **velocity * dt;
    }
}
