use godot::{engine::RenderingServer, prelude::Rid};

pub mod rid_server;

pub trait ResourceId: Clone + Copy {
    fn get_rid(&self) -> Rid;
}

#[derive(Debug, Clone, Copy)]
pub enum RenderableType {
    Camera(Rid),
    Viewport(Rid),
}

#[derive(Debug, Clone, Copy)]
pub struct Renderable(RenderableType);

impl ResourceId for Renderable {
    fn get_rid(&self) -> Rid {
        match self.0 {
            RenderableType::Camera(rid) => rid,
            RenderableType::Viewport(rid) => rid,
        }
    }
}

impl Renderable {
    pub fn create_camera() -> Self {
        Self(RenderableType::Camera(
            RenderingServer::singleton().camera_create(),
        ))
    }
    pub fn create_viewport() -> Self {
        Self(RenderableType::Viewport(
            RenderingServer::singleton().viewport_create(),
        ))
    }
    pub fn from_existing(existing: RenderableType) -> Self {
        Self(existing)
    }
}
