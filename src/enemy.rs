use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_rapier3d::prelude::*;

use crate::character::{CharacterAnimationState, CharacterAnimations, CharacterBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(YamlAssetPlugin::<EnemyData>::new(&["enemy"]))
            .add_startup_system(load)
            .add_system(spawn);
    }
}

#[derive(Component, Deref)]
struct SpawnEnemy(Handle<EnemyData>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    let data = ass.load("enemy.enemy");
    commands.spawn((SpawnEnemy(data),));
}

fn spawn(
    mut commands: Commands,
    ass: Res<AssetServer>,
    spawns: Query<(Entity, &SpawnEnemy)>,
    data: Res<Assets<EnemyData>>,
) {
    for (entity, spawn) in spawns.iter() {
        if let Some(data) = data.get(&**spawn) {
            commands.entity(entity).despawn_recursive();
            commands
                .spawn(EnemyBundle {
                    character: CharacterBundle {
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(10.0, 1.1, 10.0),
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
                    },
                    la: LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED,
                    enemy: Enemy,
                })
                .insert((Velocity::default(), Dominance::group(1)));
        }
    }
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "49c74e8a-06c3-4737-84bd-fac68b7f4469"]
struct EnemyData {
    model: String,
    idle_animation: String,
    run_animation: String,
}

#[derive(Bundle)]
struct EnemyBundle {
    #[bundle]
    character: CharacterBundle,
    la: LockedAxes,
    enemy: Enemy,
}

#[derive(Component)]
pub struct Enemy;
