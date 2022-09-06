use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_rapier3d::prelude::*;

use crate::{gltf::SpawnGltfScene, movement::Destination};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<PlayerData>::new(&["player"]))
            .add_startup_system(load)
            .add_system(spawn)
            .add_system(initial_set_animation)
            .add_system(set_animation)
            .add_system(setup_animation_player)
            .add_system(start_running)
            .add_system_to_stage(CoreStage::PostUpdate, stop_running);
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
                    transform: Transform::from_xyz(0.0, 1.1, 0.0),
                    ..default()
                },
                scene: ass.load(&data.model).into(),
                animations: CharacterAnimations {
                    idle: ass.load(&format!("{}#Animation0", data.idle_animation)),
                    run: ass.load(&format!("{}#Animation0", data.run_animation)),
                },
                animation_state: CharacterAnimationState::Idle,
                collider: Collider::cylinder(1.0, 0.5),
                rb: RigidBody::Dynamic,
                la: LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED,
                player: Player,
            });
        }
    }
}

fn initial_set_animation(
    characters: Query<
        (
            &CharacterAnimations,
            &AnimationPlayerReference,
            &CharacterAnimationState,
        ),
        Added<AnimationPlayerReference>,
    >,
    mut players: Query<&mut AnimationPlayer, Without<AnimationPlayerReference>>,
) {
    for (animations, AnimationPlayerReference(player), state) in characters.iter() {
        players
            .get_mut(*player)
            .unwrap()
            .play(animations.get_handle(state))
            .repeat();
    }
}

fn set_animation(
    characters: Query<
        (
            &CharacterAnimations,
            &AnimationPlayerReference,
            &CharacterAnimationState,
        ),
        Changed<CharacterAnimationState>,
    >,
    mut players: Query<&mut AnimationPlayer, Without<AnimationPlayerReference>>,
) {
    for (animations, AnimationPlayerReference(player), state) in characters.iter() {
        players
            .get_mut(*player)
            .unwrap()
            .play(animations.get_handle(state))
            .repeat();
    }
}

fn start_running(mut characters: Query<&mut CharacterAnimationState, Added<Destination>>) {
    for mut character in characters.iter_mut() {
        *character = CharacterAnimationState::Running;
    }
}

fn stop_running(
    removed: RemovedComponents<Destination>,
    mut characters: Query<&mut CharacterAnimationState>,
) {
    for entity in removed.iter() {
        if let Ok(mut state) = characters.get_mut(entity) {
            *state = CharacterAnimationState::Idle;
        }
    }
}

#[derive(Component)]
struct AnimationPlayerReference(Entity);

fn setup_animation_player(
    mut commands: Commands,
    player: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    parents: Query<&Parent, Without<AnimationPlayer>>,
) {
    for (player, player_parent) in player.iter() {
        commands
            .entity(**parents.get(**player_parent).unwrap())
            .insert(AnimationPlayerReference(player));
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "a3b4779e-4090-434d-bf69-0ed5b3068e76"]
struct PlayerData {
    model: String,
    idle_animation: String,
    run_animation: String,
}

#[derive(Bundle)]
struct PlayerBundle {
    #[bundle]
    spatial: SpatialBundle,
    scene: SpawnGltfScene,
    animations: CharacterAnimations,
    animation_state: CharacterAnimationState,
    collider: Collider,
    rb: RigidBody,
    la: LockedAxes,
    player: Player,
}

#[derive(Component)]
pub struct CharacterAnimations {
    idle: Handle<AnimationClip>,
    run: Handle<AnimationClip>,
}

impl CharacterAnimations {
    fn get_handle(&self, state: &CharacterAnimationState) -> Handle<AnimationClip> {
        match state {
            CharacterAnimationState::Idle => self.idle.clone(),
            CharacterAnimationState::Running => self.run.clone(),
        }
    }
}

#[derive(Component)]
enum CharacterAnimationState {
    Idle,
    Running,
}

#[derive(Component)]
pub struct Player;
