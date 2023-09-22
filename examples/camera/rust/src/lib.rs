use godot::prelude::*;
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::*,
    rendering::{
        components::{camera::CameraBundle, viewport::Viewport},
        resource::RenderingServerResources,
        traits::Renderable,
    },
};

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

        ecs.add_systems(Ready, systems::setup_main_cam)
            .add_systems(Process, systems::process_system)
            .add_systems(PhysicsProcess, systems::physics_system)
            .add_systems(ExitTree, systems::on_exit);

        ecs.get_world_mut()
            .insert_resource(RenderingServerResources::new());

        Self { base, ecs }
    }
    fn enter_tree(&mut self) {
        let root_viewport = self
            .base
            .get_viewport()
            .unwrap()
            .get_viewport_rid()
            .to_valid_u64();

        let rendering_resources = self.ecs.get_world_mut().resource_mut::<RenderingServerResources>();

        self.ecs.get_world_mut().spawn(CameraBundle {
            camera: todo!(),
            viewport: todo!(),
            transform_3d: todo!(),
        });
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
