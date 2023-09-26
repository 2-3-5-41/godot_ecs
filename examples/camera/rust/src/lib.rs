use components::MainViewport;
use godot::prelude::*;
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::*,
    resources::{
        renderable::{Renderable, RenderableType},
        rid_server::RidServer,
    },
};

mod components;
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

        ecs.add_systems(EnterTree, systems::setup_main_cam)
            .add_systems(ExitTree, systems::on_exit);

        ecs.get_world_mut()
            .insert_resource(RidServer::<Renderable>::new());

        Self { base, ecs }
    }
    fn enter_tree(&mut self) {
        // Let's go ahead and create a main viewport entity while we have access to it.
        {
            let root_viewport = self.base.get_viewport().unwrap().get_viewport_rid();

            let world = self.ecs.get_world_mut();
            let mut server = world.resource_mut::<RidServer<Renderable>>();
            let main_viewport =
                server.add(Renderable::new(RenderableType::Viewport(root_viewport)));

            world.spawn((main_viewport, MainViewport));
        }

        self.ecs.run_schedule(EnterTree)
    }
    fn ready(&mut self) {
        self.ecs.run_schedule(Ready)
    }
    fn process(&mut self, _delta: f64) {
        self.ecs.run_schedule(Process)
    }
    fn physics_process(&mut self, _delta: f64) {
        self.ecs.run_schedule(PhysicsProcess)
    }
    fn exit_tree(&mut self) {
        self.ecs.run_schedule(ExitTree)
    }
}

#[godot_api]
impl EcsWorld {}
