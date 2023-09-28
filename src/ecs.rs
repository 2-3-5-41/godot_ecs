use bevy_ecs::{
    schedule::{IntoSystemConfigs, Schedule, ScheduleLabel, Schedules},
    system::Resource,
    world::World,
};

use crate::godot_schedule::*;

pub struct Ecs {
    world: World,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    /// A simple re-implementation of a `bevy_app::app::App::add_systems()`
    pub fn add_systems<M>(
        &mut self,
        label: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();

        if let Some(schedule) = schedules.get_mut(&label) {
            schedule.add_systems(systems);
        } else {
            let mut new_schedule = Schedule::default();
            new_schedule.add_systems(systems);
            schedules.insert(label, new_schedule);
        }

        self
    }
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }
    pub fn run_schedule(&mut self, label: impl AsRef<dyn ScheduleLabel>) {
        self.world.run_schedule(label);
    }
}

impl Default for Ecs {
    fn default() -> Self {
        let mut world = World::new();

        world.add_schedule(Schedule::default(), EnterTree);
        world.add_schedule(Schedule::default(), Ready);
        world.add_schedule(Schedule::default(), Process);
        world.add_schedule(Schedule::default(), PhysicsProcess);
        world.add_schedule(Schedule::default(), ExitTree);

        Self { world }
    }
}
