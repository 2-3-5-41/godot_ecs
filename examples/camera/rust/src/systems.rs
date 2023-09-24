use bevy_ecs::system::{Commands, Query, ResMut};
use godot::{engine::RenderingServer, prelude::*};
use godot_ecs::resources::{
    rid_server::{ResourceHandle, RidServer},
    Renderable, ResourceId,
};

use crate::components::{MainCamera, MainViewport};

pub fn setup_main_cam(
    query_main_viewport: Query<(&ResourceHandle, &MainViewport)>,
    mut renderable_server: ResMut<RidServer<Renderable>>,
    mut commands: Commands,
) {
    let (viewport_handle, _) = query_main_viewport.get_single().unwrap();

    let viewport = renderable_server
        .try_get(viewport_handle)
        .expect("There is no viewport here..."); // BUG: Panicing even though `viewport_handle` does exist on the hashmap.
    let camera = Renderable::create_camera();

    RenderingServer::singleton().viewport_attach_camera(viewport.get_rid(), camera.get_rid());

    commands.spawn((renderable_server.add(camera), MainCamera));
}

pub fn process_system() {
    godot_print!("Process...")
}

pub fn physics_system() {
    godot_print!("Physics...")
}

// This system should be able to find all created RID's
// and free them on exit being called form `_exit_tree()`.
pub fn on_exit() {
    godot_print!("Attempting to free all RID's in the rendering server...");
}
