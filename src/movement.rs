use bevy::{math::Vec3Swizzles, prelude::*};

pub struct Movement;

impl Plugin for Movement {
    fn build(&self, app: &mut App) {
        app.add_system(move_towards_destination);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Destination(pub Entity);

pub fn move_towards_destination(
    mut commands: Commands,
    time: Res<Time>,
    mut destinations: Query<(Entity, &mut Transform, &Destination)>,
    targets: Query<&GlobalTransform, Without<Destination>>,
) {
    for (entity, mut moving, destination) in destinations.iter_mut() {
        if let Ok(target) = targets.get(**destination) {
            let target = target.translation().xz();
            let full_movement = target - moving.translation.xz();
            let mut movement = full_movement.normalize_or_zero() * time.delta_seconds() * 5.0;
            movement = movement.clamp_length_max(full_movement.length());
            if movement.length() <= f32::EPSILON {
                // Reached destination, remove everything
                commands.entity(entity).remove::<Destination>();
                commands.entity(**destination).despawn_recursive();
            } else {
                moving.rotation = Quat::from_rotation_arc(
                    Vec3::Z,
                    full_movement.extend(0.0).xzy().normalize_or_zero(),
                );
                moving.translation += movement.extend(0.0).xzy();
            }
        } else {
            // Destination does not exist or does not work as a destination, remove everything
            commands.entity(entity).remove::<Destination>();
            commands.entity(**destination).despawn_recursive();
        }
    }
}
