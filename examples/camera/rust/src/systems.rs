use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res, ResMut},
};
use godot::{engine::RenderingServer, prelude::*};
use godot_ecs::resources::{
    renderable::{camera::Camera, viewport::Viewport},
    rid_server::{ResourceHandle, RidServer},
    traits::ResourceId,
};

use crate::{components::*, resources::DeltaTime};

/// As the name implies, we are going to create our camera resource here, through the
/// [`RidServer`], it will also create the [`Rid`] in the [`RenderingServer`] then return
/// a useful [`ResourceHandle`] that we can then store as a component on our entity.
///
/// I'd recommend that you put systems like this in the [`EnterTree`] stage.
pub fn create_main_cam(mut camera_server: ResMut<RidServer<Camera>>, mut commands: Commands) {
    let camera_pos = Position(Vector3::new(1.0, 2.0, -1.0));
    let camera = Camera::create();

    commands.spawn((
        camera_server.add(camera),
        camera_pos,
        Velocity(Vector3::new(0.0, 0.0, -1.0)),
        MainCamera,
    ));
}

/// As the name implies, this system will setup our main camera, making it renders to our main viewport,
/// all of this being done with easy builder patterns that interact with the [`RenderingServer`] in the background.
pub fn setup_main_cam(
    camera_server: Res<RidServer<Camera>>,
    camera_query: Query<&ResourceHandle, (With<Position>, With<Velocity>, With<MainCamera>)>,
    viewport_server: Res<RidServer<Viewport>>,
    viewport_query: Query<&ResourceHandle, With<MainViewport>>,
) {
    let Ok(viewport_handle) = viewport_query.get_single() else {
        println!("No main viewport entity found!");
        return;
    };
    let Ok(camera_handle) = camera_query.get_single() else {
        println!("No main camera entity found!");
        return;
    };

    let camera = camera_server
        .get(camera_handle)
        .set_perspective(90.0, 0.1, 4000.0);

    let viewport = viewport_server.get(viewport_handle);
    RenderingServer::singleton().viewport_attach_camera(viewport.get_rid(), camera.get_rid())
}

/// As the name implies, we are grabbing our main camera entity, and accessing it's [`ResourceHandle`] from the [`RidServer`],
/// then applying a simple constant velocity to it's position, then passing that to the `set_transform` funciton which then
/// sends that to the [`RenderingServer`] to update the camera's position in world space.
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

    camera_server.get(camera).set_transform(
        Transform3D::new(Basis::IDENTITY, position.0).looking_at(Vector3::ZERO, Vector3::UP, false),
    );
}

/// This system, when called on exit, will tell the [`RidServer`] to free all [`Rid`]s.
pub fn on_exit(mut camera_server: ResMut<RidServer<Camera>>) {
    godot_print!("Attempting to free all RID's in the rendering server... \n");
    camera_server.free_all()
}
