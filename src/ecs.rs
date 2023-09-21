use std::collections::HashMap;

use bevy_ecs::{schedule::Schedule, world::World};

pub struct Ecs {
    world: World,
    schedules: HashMap<ScheduleIdentifier, Schedule>,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            schedules: HashMap::new(),
        }
    }
    pub fn get_world(&self) -> &World {
        &self.world
    }
    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    pub fn add_schedule(&mut self, identifier: ScheduleIdentifier, schedule: Schedule) {
        let map = &mut self.schedules;
        map.insert(identifier, schedule);
    }
    pub fn try_get_schedule(&mut self, identifier: ScheduleIdentifier) -> Option<&mut Schedule> {
        let map = &mut self.schedules;
        map.get_mut(&identifier)
    }
    /// Can panic!
    pub fn get_schedule(&mut self, identifier: ScheduleIdentifier) -> &mut Schedule {
        self.try_get_schedule(identifier).unwrap()
    }
    pub fn run_schedule(&mut self, identifier: ScheduleIdentifier) {
        let map = &mut self.schedules;
        for (id, schedule) in map {
            if id == &identifier {
                schedule.run(&mut self.world)
            }
        }
    }
}

impl Default for Ecs {
    fn default() -> Self {
        let mut hashmap: HashMap<ScheduleIdentifier, Schedule> = HashMap::new();

        hashmap.insert(ScheduleIdentifier::Startup, Schedule::default());
        hashmap.insert(ScheduleIdentifier::Process, Schedule::default());
        hashmap.insert(ScheduleIdentifier::PhysicsProcess, Schedule::default());

        Self {
            world: World::new(),
            schedules: hashmap,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ScheduleIdentifier {
    Startup,
    Process,
    PhysicsProcess,
}
