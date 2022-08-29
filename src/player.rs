use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_rapier3d::prelude::*;

use crate::gltf::SpawnGltfScene;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<PlayerData>::new(&["player"]))
            .add_startup_system(load)
            .add_system(spawn)
            .add_system(initial_animate_character);
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
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                },
                scene: ass.load(&data.model).into(),
                animations: CharacterAnimations {
                    idle: ass.load(&format!("{}#Animation0", data.idle_animation)),
                },
                collider: Collider::cylinder(1.0, 0.5),
                rb: RigidBody::Dynamic,
                la: LockedAxes::TRANSLATION_LOCKED_Y
                    | LockedAxes::ROTATION_LOCKED_X
                    | LockedAxes::ROTATION_LOCKED_Z,
            });
        }
    }
}

fn initial_animate_character(
    mut players: Query<(&Parent, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&Parent, Without<AnimationPlayer>>,
    animations: Query<&CharacterAnimations, Without<AnimationPlayer>>,
) {
    for (mut parent, mut player) in players.iter_mut() {
        loop {
            if let Ok(animations) = animations.get(parent.get()) {
                player.play(animations.idle.clone()).repeat();
                break;
            } else if let Ok(new_parent) = parents.get(parent.get()) {
                parent = new_parent;
            } else {
                break;
            }
        }
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "a3b4779e-4090-434d-bf69-0ed5b3068e76"]
struct PlayerData {
    model: String,
    idle_animation: String,
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    spatial: SpatialBundle,
    scene: SpawnGltfScene,
    animations: CharacterAnimations,
    collider: Collider,
    rb: RigidBody,
    la: LockedAxes,
}

#[derive(Component)]
struct CharacterAnimations {
    idle: Handle<AnimationClip>,
}
