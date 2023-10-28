use bevy_ecs::schedule::IntoSystemConfigs;
use godot::prelude::*;
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::{Process, Ready},
    xr::server::XrServer,
};

mod components;
mod systems;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct XrEcs {
    ecs: Ecs,

    #[allow(dead_code)]
    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl Node3DVirtual for XrEcs {
    fn init(base: Base<Node3D>) -> Self {
        let mut ecs = Ecs::default();

        ecs.insert_resource(XrServer);

        ecs.add_systems(
            Ready,
            (systems::init_xr, systems::create_ecs_controllers).chain(),
        )
        .add_systems(Process, systems::read_ecs_controller_positions);

        Self { ecs, base }
    }

    fn ready(&mut self) {
        self.ecs.run_schedule(Ready)
    }

    fn process(&mut self, _delta: f64) {
        self.ecs.run_schedule(Process)
    }
}

#[godot_api]
impl XrEcs {}
