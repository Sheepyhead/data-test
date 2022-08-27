use bevy::{prelude::*, gltf::Gltf};

pub struct GltfPlugin;

impl Plugin for GltfPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_gltf);
    }
}


impl From<Handle<Gltf>> for SpawnGltfScene {
    fn from(handle: Handle<Gltf>) -> Self {
        Self(handle)
    }
}

#[derive(Component, Deref)]
pub struct SpawnGltfScene(pub Handle<Gltf>);

fn spawn_gltf(
    mut commands: Commands,
    scenes: Query<(Entity, &SpawnGltfScene)>,
    gltfs: Res<Assets<Gltf>>,
) {
    for (entity, SpawnGltfScene(handle)) in scenes.iter() {
        if let Some(gltf) = gltfs.get(handle) {
            commands
                .entity(entity)
                .insert(gltf.scenes[0].clone())
                .remove::<SpawnGltfScene>();
        }
    }
}