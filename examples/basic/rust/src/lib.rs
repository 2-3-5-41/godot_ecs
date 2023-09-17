use bevy_ecs::{schedule::Schedule, system::{Query, Commands}, world::World};
use godot::prelude::{
    gdextension, godot_api, godot_print, Base, Basis, ExtensionLibrary, GodotClass, Node,
    NodeVirtual, Transform3D, Vector3, Color,
};
use godot_ecs::{
    rendering::{camera::Camera, viewport::Viewport, canvas::Canvas},
    schedule_labels::{PhysicsUpdate, Startup, Update},
};

struct BasicECS;

#[gdextension]
unsafe impl ExtensionLibrary for BasicECS {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Example3D {
    #[base]
    base: Base<Node>,
    world: Option<World>,
}

// This is our "Root" node, this is where we will control the game loop, assets, and other
// fun game magic. This node will be the powerhouse that utilizes Godot's internal data driven
// servers, as well as pair those internals with bevy_ecs components and resources.
#[godot_api]
impl NodeVirtual for Example3D {
    fn init(base: Base<Node>) -> Self {
        // Define your world and schedule(s).
        let mut world = World::new();
        let mut startup_schedule = Schedule::default();
        let mut update_schedule = Schedule::default();
        let mut physics_schedule = Schedule::default();

        //Add systems to schedule(s).
        startup_schedule.add_systems(startup_system);
        update_schedule.add_systems(update_system);
        physics_schedule.add_systems(physics_update_system);

        // Add schedules to world.
        world.add_schedule(startup_schedule, Startup);
        world.add_schedule(update_schedule, Update);
        world.add_schedule(physics_schedule, PhysicsUpdate);

        Self {
            base,
            world: Some(world),
        }
    }

    fn process(&mut self, _delta: f64) {
        self.get_world().run_schedule(Update);
    }
    fn physics_process(&mut self, _delta: f64) {
        self.get_world().run_schedule(PhysicsUpdate);
    }
    fn enter_tree(&mut self) {
        // We may want to add these in early to access them later.
        let root_viewport = self.base.get_viewport().unwrap().get_viewport_rid();
        self.get_world().spawn(Viewport::from_rid(root_viewport));
        self.get_world()
            .spawn(Camera::new().set_perspective(90.0, 0.1, 4000.0));
    }
    fn ready(&mut self) {
        self.get_world().run_schedule(Startup);
    }
}

#[godot_api]
impl Example3D {
    fn get_world(&mut self) -> &mut World {
        self.world.as_mut().unwrap()
    }
}

fn startup_system(query_root: Query<&Viewport>, query_main_cam: Query<&Camera>, mut commands: Commands) {
    let root = query_root
        .get_single()
        .expect("There is no root viewport...");
    let main_cam = query_main_cam
        .get_single()
        .expect("There is no main camera...");

    root.attach_camera(main_cam.get_rid());
    main_cam
        .set_perspective(90.0, 0.1, 4000.0)
        .set_transform(Transform3D::new(
            Basis::IDENTITY,
            Vector3::new(1.0, 1.0, 1.0),
        ));

    godot_print!("Ready!");

    commands.spawn(Canvas::new().set_modulate(Color::from_rgb(255.0, 100.0, 50.0)));
}

fn update_system(canvas_query: Query<&Canvas>) {
    canvas_query.iter().for_each(|canvas| {
        godot_print!("Found a canvas: {:?}", canvas);
    });
    godot_print!("Update...");
}

fn physics_update_system() {
    godot_print!("Physics update...")
}
