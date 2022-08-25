use std::collections::BTreeMap;

use bevy::{gltf::Gltf, prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;

pub struct Tiles;

impl Plugin for Tiles {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<Tile>::new(&["tile"]))
            .add_plugin(YamlAssetPlugin::<RepeatingTileSet>::new(&["tileset"]))
            .add_startup_system(load)
            .add_system(spawn_tileset)
            .add_system(spawn_gltf);
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b0f80785-1e5b-493a-a29f-b8e20863989a"]
pub struct Tile {
    model: String,
    position: BTreeMap<String, i32>,
}

// struct TileSet(Vec<Handle<Tile>>);

struct TileSets(Vec<Handle<RepeatingTileSet>>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    let grass_set = ass.load("grasstileset.tileset");
    commands.insert_resource(TileSets(vec![grass_set]));
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
            spawn_tile(
                &tile.model,
                IVec2 {
                    x: *tile.position.get("x").unwrap(),
                    y: *tile.position.get("y").unwrap(),
                },
                &mut commands,
                &ass,
            );
        }
    }
}

fn spawn_tile(model: &String, pos: IVec2, commands: &mut Commands, ass: &Res<AssetServer>) {
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(pos.x as f32, 0.0, pos.y as f32),
            ..default()
        })
        .insert(SpawnGltfScene(ass.load(model)));
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

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "688ebe3a-8d7f-4658-b945-f408c1370ba8"]
struct RepeatingTileSet {
    model: String,
    width: i32,
    height: i32,
}

fn spawn_tileset(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut events: EventReader<AssetEvent<RepeatingTileSet>>,
    sets: Res<Assets<RepeatingTileSet>>,
) {
    for ev in events.iter() {
        if let AssetEvent::Created { handle } = ev {
            let set = sets.get(handle).unwrap();
            for x in 0..set.width {
                for y in 0..set.height {
                    spawn_tile(&set.model, IVec2 { x, y }, &mut commands, &ass);
                }
            }
        }
    }
}
