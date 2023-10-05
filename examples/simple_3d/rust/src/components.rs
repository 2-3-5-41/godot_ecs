use bevy_ecs::component::Component;
use godot::prelude::Vector3;

#[derive(Component, Debug, Clone)]
pub struct MainViewport;

#[derive(Component, Debug, Clone)]
pub struct MainCamera;

#[derive(Component, Debug, Clone)]
pub struct MainScenario;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vector3);

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vector3);
