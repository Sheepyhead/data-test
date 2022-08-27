use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::gltf::SpawnGltfScene;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<PlayerData>::new(&["player"]))
            .add_startup_system(load)
            .add_system(spawn);
    }
}

#[derive(Component, Deref)]
struct SpawnPlayer(Handle<PlayerData>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    let data = ass.load("archer.player");
    commands.spawn_bundle((SpawnPlayer(data),));
}

fn spawn(
    mut commands: Commands,
    ass: Res<AssetServer>,
    spawns: Query<(Entity, &SpawnPlayer)>,
    data: Res<Assets<PlayerData>>,
) {
    for (entity, spawn) in spawns.iter() {
        if let Some(data) = data.get(&**spawn) {
            commands.entity(entity).despawn_recursive();
            commands.spawn_bundle(PlayerBundle {
                spatial: default(),
                scene: ass.load(&data.model).into(),
            });
        }
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "a3b4779e-4090-434d-bf69-0ed5b3068e76"]
struct PlayerData {
    model: String,
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    spatial: SpatialBundle,
    scene: SpawnGltfScene,
}
