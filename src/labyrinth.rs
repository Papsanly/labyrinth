use bevy::prelude::*;

pub struct LabyrinthPlugin;

#[derive(Component)]
pub struct Labyrinth;

impl Plugin for LabyrinthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_plane);
    }
}

fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(StandardMaterial::default()),
            ..default()
        },
        Labyrinth,
    ));
}
