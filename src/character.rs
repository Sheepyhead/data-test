use bevy::prelude::*;

use crate::movement::Destination;

pub struct Character;

impl Plugin for Character {
    fn build(&self, app: &mut App) {
        app.add_system(initial_set_animation)
            .add_system(set_animation)
            .add_system(setup_animation_player)
            .add_system(start_running)
            .add_system_to_stage(CoreStage::PostUpdate, stop_running);
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
#[derive(Component)]
pub struct CharacterAnimations {
    pub idle: Handle<AnimationClip>,
    pub run: Handle<AnimationClip>,
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
pub enum CharacterAnimationState {
    Idle,
    Running,
}
