use std::collections::BTreeMap;

use bevy::{gltf::Gltf, prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;

pub struct Tiles;

impl Plugin for Tiles {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<Tile>::new(&["tile"]))
            .add_startup_system(load)
            .add_system(spawn)
            .add_system(spawn_gltf);
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b0f80785-1e5b-493a-a29f-b8e20863989a"]
pub struct Tile {
    model: String,
    position: BTreeMap<String, i32>,
}

struct TileSet(Vec<Handle<Tile>>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    let grass_tile = ass.load("grasstile.tile");
    commands.insert_resource(TileSet(vec![grass_tile]));
}

#[derive(Component, Deref)]
struct SpawnGltfScene(Handle<Gltf>);

fn spawn(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut events: EventReader<AssetEvent<Tile>>,
    tiles: Res<Assets<Tile>>,
) {
    for ev in events.iter() {
        if let AssetEvent::Created { handle } = ev {
            let tile = tiles.get(handle).unwrap();
            commands
                .spawn_bundle(SpatialBundle {
                    transform: Transform::from_xyz(
                        *tile.position.get("x").unwrap() as f32,
                        0.0,
                        *tile.position.get("y").unwrap() as f32,
                    ),
                    ..default()
                })
                .insert(SpawnGltfScene(ass.load(&tile.model)));
        }
    }
}

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
