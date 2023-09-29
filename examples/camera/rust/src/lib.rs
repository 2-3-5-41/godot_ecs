use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs};
use components::MainViewport;
use godot::prelude::*;
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::*,
    resources::{
        renderable::{camera::Camera, viewport::Viewport},
        rid_server::RidServer,
    },
};
use resources::DeltaTime;

mod components;
mod resources;
mod systems;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct EcsWorld {
    #[base]
    base: Base<Node>,
    ecs: Ecs,
}

#[godot_api]
impl NodeVirtual for EcsWorld {
    fn init(base: Base<Node>) -> Self {
        let mut ecs = Ecs::default();

        ecs.add_systems(
            EnterTree,
            (
                systems::create_main_cam,
                apply_deferred,
                systems::setup_main_cam,
            )
                .chain(),
        )
        .add_systems(Process, systems::move_camera)
        .add_systems(ExitTree, systems::on_exit)
        .insert_resource(RidServer::<Viewport>::new())
        .insert_resource(RidServer::<Camera>::new())
        .insert_resource(DeltaTime(0.0));

        Self { base, ecs }
    }
    fn enter_tree(&mut self) {
        // Let's go ahead and create a main viewport entity while we have access to it.
        {
            let root_viewport = self.base.get_viewport().unwrap().get_viewport_rid();

            let world = self.ecs.get_world_mut();
            let mut server = world.resource_mut::<RidServer<Viewport>>();
            let main_viewport = server.add(Viewport::from_rid(root_viewport));

            world.spawn((main_viewport, MainViewport));
        }

        self.ecs.run_schedule(EnterTree)
    }
    fn ready(&mut self) {
        self.ecs.run_schedule(Ready)
    }
    fn process(&mut self, delta: f64) {
        // Set process delta time.
        self.ecs.get_world_mut().resource_mut::<DeltaTime>().0 = delta;

        // Then run process schedule.
        self.ecs.run_schedule(Process);
    }
    fn physics_process(&mut self, delta: f64) {
        // Set process delta time.
        self.ecs.get_world_mut().resource_mut::<DeltaTime>().0 = delta;

        // Then run physics process schedule.
        self.ecs.run_schedule(PhysicsProcess)
    }
    fn exit_tree(&mut self) {
        self.ecs.run_schedule(ExitTree)
    }
}

#[godot_api]
impl EcsWorld {}
