use bevy_ecs::component::Component;
use godot::prelude::Transform3D;

#[derive(Component, Debug, Clone)]
pub struct MainViewport;

#[derive(Component, Debug, Clone)]
pub struct MainCamera;

#[derive(Component, Debug, Clone, Copy)]
pub struct GdTransform3D(pub Transform3D);
