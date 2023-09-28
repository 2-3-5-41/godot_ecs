use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res, ResMut},
};
use godot::{engine::RenderingServer, prelude::*};
use godot_ecs::resources::{
    renderable::{camera::Camera, viewport::Viewport},
    rid_server::{ResourceHandle, RidServer}, traits::ResourceId
};

use crate::{components::*, resources::DeltaTime};

pub fn setup_main_cam(
    query_main_viewport: Query<&ResourceHandle, With<MainViewport>>,
    viewport_server: Res<RidServer<Viewport>>,
    mut camera_server: ResMut<RidServer<Camera>>,
    mut commands: Commands,
) {
    let viewport_handle = query_main_viewport.get_single().unwrap();

    let viewport = viewport_server
        .try_get(viewport_handle)
        .expect("There is no viewport here...");
    let camera = Camera::create();
    let camera_pos = Position(Vector3::new(1.0, 2.0, -1.0));

    RenderingServer::singleton().viewport_attach_camera(viewport.get_rid(), camera.get_rid());
    RenderingServer::singleton().camera_set_transform(
        camera.get_rid(),
        Transform3D::new(Basis::IDENTITY, camera_pos.0),
    );

    commands.spawn((
        camera_server.add(camera),
        camera_pos,
        Velocity(Vector3::new(0.0, 0.0, -1.0)),
        MainCamera,
    ));
}

pub fn move_camera(
    camera_server: ResMut<RidServer<Camera>>,
    delta_time: Res<DeltaTime>,
    mut camera_query: Query<(&ResourceHandle, &mut Position, &Velocity), With<MainCamera>>,
) {
    let Ok((camera, mut position, velocity)) = camera_query.get_single_mut() else {
        godot_error!("Failed to get main camera!");
        return;
    };

    position.0.z += velocity.0.z * delta_time.as_f32();

    RenderingServer::singleton().camera_set_transform(
        camera_server.get(camera).get_rid(),
        Transform3D::new(Basis::IDENTITY, position.0).looking_at(Vector3::ZERO, Vector3::UP, false),
    )
}

// This system should be able to find all created RID's
// and free them on exit being called form `_exit_tree()`.
pub fn on_exit(mut camera_server: ResMut<RidServer<Camera>>) {
    godot_print!("Attempting to free all RID's in the rendering server...");
    camera_server.free_all()
}
