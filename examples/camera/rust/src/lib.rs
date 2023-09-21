use bevy_ecs::system::Query;
use godot::{prelude::*, engine::RenderingServer};
use godot_ecs::{
    ecs::{Ecs, ScheduleIdentifier},
    rendering::{
        components::{camera::{CameraBundle, Camera}, viewport::Viewport},
        Renderable,
    }, math::transform::GdTransform3D,
};

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

        // In `init`, you can go ahead and add your systems to the [`Ecs`] schedules.
        ecs.get_schedule(ScheduleIdentifier::Startup)
            .add_systems(setup_main_cam);

        ecs.get_schedule(ScheduleIdentifier::Process)
            .add_systems(process_system);

        ecs.get_schedule(ScheduleIdentifier::PhysicsProcess)
            .add_systems(physics_system);

        Self { base, ecs }
    }
    fn enter_tree(&mut self) {
        let root_viewport = self.base
            .get_viewport()
            .unwrap()
            .get_viewport_rid()
            .to_valid_u64();

        self.ecs.get_world_mut()
            .spawn(CameraBundle::with_viewport(Viewport::new_from_existing(
                root_viewport,
            )));
    }
    fn ready(&mut self) {
        self.ecs.run_schedule(ScheduleIdentifier::Startup);
    }
    fn process(&mut self, _delta: f64) {
        self.ecs.run_schedule(ScheduleIdentifier::Process);
    }
    fn physics_process(&mut self, _delta: f64) {
        self.ecs.run_schedule(ScheduleIdentifier::PhysicsProcess);
    }
}

#[godot_api]
impl EcsWorld {}

fn setup_main_cam(mut query: Query<(&Camera, &Viewport, &mut GdTransform3D)>) {
    query.iter_mut().for_each(|(camera, viewport, mut transform)| {
        transform.translate(Vector3::new(0.0, 1.0, -1.0));

        let cam_rid = camera.get_resource().get_rid();
        let viewport_rid = viewport.get_resource().get_rid();

        RenderingServer::singleton().viewport_attach_camera(Rid::new(viewport_rid), Rid::new(cam_rid));
    });
    godot_print!("Attempted to attach new 'main' camera to root viewport...");
}

fn process_system() {
    godot_print!("Process...")
}

fn physics_system() {
    godot_print!("Physics...")
}
