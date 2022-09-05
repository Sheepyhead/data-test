use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera3dBundle::default();

    camera.transform.translation = Vec3::splat(10.0);
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
}
