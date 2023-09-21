use crate::rendering::{
    resource::{RendererCreate, RendererResource},
    Renderable,
};
use bevy_ecs::component::Component;

#[derive(Component, Debug, Clone, Copy)]
pub struct Viewport {
    resource: RendererResource,
}

impl Renderable for Viewport {
    fn new() -> Self {
        Self {
            resource: RendererResource::create(RendererCreate::Viewport),
        }
    }
    fn get_resource(&self) -> &RendererResource {
        &self.resource
    }

    fn new_from_existing(rid: Option<u64>) -> Self {
        Self {
            resource: RendererResource::from_existing(rid).unwrap(),
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
