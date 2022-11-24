use bevy::{prelude::*, transform::TransformSystem};

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<CameraOffset>(Vec3::splat(10.0).into())
            .add_startup_system(spawn_camera)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                follow_player.after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Deref, DerefMut, Resource)]
pub struct CameraOffset(Vec3);

impl From<Vec3> for CameraOffset {
    fn from(vec: Vec3) -> Self {
        Self(vec)
    }
}

fn spawn_camera(mut commands: Commands, offset: Res<CameraOffset>) {
    let mut camera = Camera3dBundle::default();

    camera.transform.translation = **offset;
    camera.transform.look_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(camera);
}

fn follow_player(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    player: Query<&GlobalTransform, (With<Player>, Changed<GlobalTransform>)>,
    offset: Res<CameraOffset>,
) {
    if let Ok(target) = player.get_single() {
        for mut camera in cameras.iter_mut() {
            camera.translation = target.translation() + **offset;
        }
    }
}
