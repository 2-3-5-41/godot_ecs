use bevy::ecs::schedule::ScheduleLabel;

#[derive(Debug, Hash, Clone, PartialEq, Eq, ScheduleLabel)]
pub struct Startup;

#[derive(Debug, Hash, Clone, PartialEq, Eq, ScheduleLabel)]
pub struct Update;

#[derive(Debug, Hash, Clone, PartialEq, Eq, ScheduleLabel)]
pub struct PhysicsUpdate;
