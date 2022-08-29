use bevy::{prelude::*, render::camera::Projection};
use bevy_rapier3d::{
    prelude::*,
    render::RapierDebugRenderPlugin,
};

use crate::common::approx_equal;

pub struct Physics;

impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system_to_stage(CoreStage::First, update_under_cursor)
            .insert_resource(UnderCursor::default());
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct UnderCursor(pub Option<Collision>);

#[derive(Debug)]
pub struct Collision {
    pub hit: Entity,
    pub intersection: Vec3,
}

fn update_under_cursor(
    windows: Res<Windows>,
    context: Res<RapierContext>,
    mut under_cursor: ResMut<UnderCursor>,
    camera: Query<(&bevy::prelude::Camera, &GlobalTransform, &Projection), With<Camera3d>>,
) {
    if let Some(cursor_pos_screen) = windows.get_primary().and_then(Window::cursor_position) {
        let (camera, camera_transform, projection) = match camera.get_single() {
            Ok(data) => data,
            Err(_) => return,
        };
        let projection = match projection {
            Projection::Perspective(persp) => persp,
            Projection::Orthographic(_) => panic!(),
        };
        let (from, to) =
            ray_from_screenspace(cursor_pos_screen, &windows, camera, projection, camera_transform, 100.0);

        if let Some((hit, RayIntersection { point, .. })) = context.cast_ray_and_get_normal(
            from,
            to,
            Real::MAX,
            false,
            QueryFilter::default().groups(InteractionGroups::all()),
        ) {
            **under_cursor = Some(Collision {
                hit,
                intersection: point,
            });
        } else {
            **under_cursor = None;
        }
    }
}


pub fn ray_from_screenspace(
    cursor_pos_screen: Vec2,
    windows: &Res<Windows>,
    camera: &Camera,
    perspective: &PerspectiveProjection,
    camera_transform: &GlobalTransform,
    length: f32,
) -> (Vec3, Vec3) {
    let view = camera_transform.compute_matrix();
    let window = windows.get_primary().unwrap();
    let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
    let projection = camera.projection_matrix();

    // 2D Normalized device coordinate cursor position from (-1, -1) to (1, 1)
    let cursor_ndc = (cursor_pos_screen / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let ndc_to_world: Mat4 = view * projection.inverse();
    let world_to_ndc = projection * view;
    let is_orthographic = approx_equal(projection.w_axis[3], 1.0);

    // Compute the cursor position at the near plane. The bevy camera looks at -Z.
    let ndc_near = world_to_ndc.transform_point3(-Vec3::Z * perspective.near).z;
    let cursor_pos_near = ndc_to_world.transform_point3(cursor_ndc.extend(ndc_near));

    // Compute the ray's direction depending on the projection used.
    let ray_direction = if is_orthographic {
        view.transform_vector3(-Vec3::Z)
    } else {
        cursor_pos_near - camera_transform.translation()
    };

    (cursor_pos_near, ray_direction * length)
}
