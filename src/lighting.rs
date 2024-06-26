use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        let sky_color = Color::hex("#9bc4eb").unwrap();
        app.insert_resource(ClearColor(sky_color))
            .insert_resource(AmbientLight {
                color: sky_color,
                brightness: 200.0,
            })
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
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
    });
}
