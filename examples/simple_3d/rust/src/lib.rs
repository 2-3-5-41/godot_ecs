use components::{MainScenario, MainViewport};
use godot::prelude::*;
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::*,
    resources::{
        renderable::{
            camera::Camera, directional_light::DirectionalLight, instance::Instance,
            scenario::Scenario, viewport::Viewport,
        },
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
#[class(base=Node3D)]
pub struct EcsWorld {
    #[base]
    base: Base<Node3D>,
    ecs: Ecs,
}

#[godot_api]
impl NodeVirtual for EcsWorld {
    fn init(base: Base<Node3D>) -> Self {
        let mut ecs = Ecs::default();

        // Add our systems.
        ecs.add_systems(EnterTree, systems::init);
        // .add_systems(Ready, systems)
        // .add_systems(Process, systems)
        // .add_systems(PhysicsProcess, systems)
        // .add_systems(ExitTree, systems)

        // Add our resources.
        ecs.insert_resource(RidServer::<Viewport>::new())
            .insert_resource(RidServer::<Camera>::new())
            .insert_resource(RidServer::<DirectionalLight>::new())
            .insert_resource(RidServer::<Instance>::new())
            .insert_resource(RidServer::<Scenario>::new())
            .insert_resource(DeltaTime(0.0));

        Self { base, ecs }
    }
    fn enter_tree(&mut self) {
        // Let's go ahead and create a main viewport entity while we have access to it,
        // if we intend on rendering to the main window of our game.
        self.store_main_viewport();

        // We also want to get the root scenario so we can create our own visual instances
        // and render them.
        self.store_scenario();
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
impl EcsWorld {
    fn store_main_viewport(&mut self) {
        let root_viewport = self.base.get_viewport().unwrap().get_viewport_rid();

        let world = self.ecs.get_world_mut();
        let mut server = world.resource_mut::<RidServer<Viewport>>();
        let main_viewport = server.add(Viewport::from_rid(root_viewport));

        world.spawn((main_viewport, MainViewport));
    }
    fn store_scenario(&mut self) {
        let root_scenario = self.base.get_world_3d().unwrap().get_scenario();

        let world = self.ecs.get_world_mut();
        let mut server = world.resource_mut::<RidServer<Scenario>>();
        let main_scenario = server.add(Scenario::from_rid(root_scenario));

        world.spawn((main_scenario, MainScenario));
    }
}
