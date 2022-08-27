use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::gltf::SpawnGltfScene;

pub struct Tiles;

impl Plugin for Tiles {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<RepeatingTileSet>::new(&["tileset"]))
            .add_startup_system(load)
            .add_system(spawn_tileset);
    }
}
struct TileSets(Vec<Handle<RepeatingTileSet>>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    let grass_set = ass.load("grass.tileset");
    commands.insert_resource(TileSets(vec![grass_set]));
}


#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "688ebe3a-8d7f-4658-b945-f408c1370ba8"]
struct RepeatingTileSet {
    model: String,
    width: u32,
    height: u32,
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
            let mut tiles = Vec::<_>::with_capacity((set.width * set.height) as usize);
            for x in 0..set.width {
                for y in 0..set.height {
                    tiles.push(TileBundle {
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                            ..default()
                        },
                        scene: ass.load(&set.model).into(),
                    });
                }
            }
            commands.spawn_batch(tiles);
        }
    }
}

#[derive(Bundle)]
struct TileBundle {
    #[bundle]
    spatial: SpatialBundle,
    scene: SpawnGltfScene,
}
