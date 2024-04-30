use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        // ThirdPersonCamera {
        //     aim_enabled: false,
        //     aim_button: MouseButton::Left,
        //     aim_speed: 0.0,
        //     aim_zoom: 0.0,
        //     cursor_lock_toggle_enabled: false,
        //     cursor_lock_active: false,
        //     cursor_lock_key: KeyCode::KeyC,
        //     gamepad_settings: Default::default(),
        //     mouse_sensitivity: 2.0,
        //     mouse_orbit_button_enabled: true,
        //     mouse_orbit_button: MouseButton::Right,
        //     offset_enabled: false,
        //     offset: Offset::new(0.8, 0.8),
        //     offset_toggle_enabled: false,
        //     offset_toggle_key: KeyCode::KeyT,
        //     offset_toggle_speed: 5.0,
        //     zoom_enabled: true,
        //     zoom: Zoom::new(1.5, 3.0),
        //     zoom_sensitivity: 1.0,
        // },
        ThirdPersonCamera::default(),
        Camera3dBundle::default(),
    ));
}
