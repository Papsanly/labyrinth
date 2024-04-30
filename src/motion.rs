use bevy::prelude::*;

const FRICTION: f32 = 0.5;

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
        let dt = time.delta_seconds();
        **velocity = **velocity + (**acceleration - FRICTION * **velocity) * dt;
        transform.translation += **velocity * dt;
    }
}
