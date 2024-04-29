use bevy::{pbr::PointLightShadowMap, prelude::*};

pub struct LightingPlugin;

#[derive(Component)]
pub struct Light;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PointLightShadowMap { size: 4096 })
            .add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                intensity: 5_000_000.0,
                ..default()
            },
            transform: Transform::from_xyz(3.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Light,
    ));
}
