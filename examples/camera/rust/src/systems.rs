use bevy_ecs::system::Query;
use godot::{engine::RenderingServer, prelude::*};
use godot_ecs::{
    math::transform::GdTransform3D,
    rendering::{
        components::{camera::Camera, viewport::Viewport},
        traits::Renderable,
    },
};

pub fn setup_main_cam(mut query: Query<(&Camera, &Viewport, &mut GdTransform3D)>) {
    query
        .iter_mut()
        .for_each(|(camera, viewport, mut transform)| {
            todo!()
        });
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
