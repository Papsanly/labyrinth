use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub struct LightingPlugin;

#[derive(Component)]
pub struct Light;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10000.0,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -FRAC_PI_2 - 0.3,
                0.4,
                0.0,
            )),
            ..default()
        },
        Light,
    ));
}
