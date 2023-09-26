use bevy_ecs::{
    query::With,
    system::{Commands, Query, ResMut},
};
use godot::{engine::RenderingServer, prelude::*};
use godot_ecs::resources::{
    renderable::{Renderable, RenderableType},
    rid_server::{ResourceHandle, RidServer},
    ResourceId,
};

use crate::components::{GdTransform3D, MainCamera, MainViewport};

pub fn setup_main_cam(
    query_main_viewport: Query<&ResourceHandle, With<MainViewport>>,
    mut renderable_server: ResMut<RidServer<Renderable>>,
    mut commands: Commands,
) {
    let viewport_handle = query_main_viewport.get_single().unwrap();

    let viewport = renderable_server
        .try_get(viewport_handle)
        .expect("There is no viewport here...");
    let camera = Renderable::new(RenderableType::Camera(
        RenderingServer::singleton().camera_create(),
    ));
    let camera_transform = GdTransform3D(
        Transform3D::new(Basis::IDENTITY, Vector3::new(0.0, 1.0, -5.0)).looking_at(
            Vector3::ZERO,
            Vector3::UP,
            false,
        ),
    );

    RenderingServer::singleton().viewport_attach_camera(viewport.get_rid(), camera.get_rid());
    RenderingServer::singleton().camera_set_transform(camera.get_rid(), camera_transform.0);

    commands.spawn((renderable_server.add(camera), camera_transform, MainCamera));
}

// This system should be able to find all created RID's
// and free them on exit being called form `_exit_tree()`.
pub fn on_exit(mut renderable_server: ResMut<RidServer<Renderable>>) {
    godot_print!("Attempting to free all RID's in the rendering server...");
    renderable_server.free_all()
}
