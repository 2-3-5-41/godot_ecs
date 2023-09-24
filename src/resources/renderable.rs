use godot::{engine::RenderingServer, prelude::Rid};

use super::ResourceId;

#[derive(Debug, Clone, Copy)]
pub enum RenderableType {
    CameraAttributes(Rid),
    Camera(Rid),
    Viewport(Rid),
}

#[derive(Debug, Clone, Copy)]
pub struct Renderable(RenderableType);

impl ResourceId for Renderable {
    fn get_rid(&self) -> Rid {
        unimplemented!()
    }
}

impl Renderable {
    pub fn create_camera_attributes() -> Self {
        Self(RenderableType::CameraAttributes(
            RenderingServer::singleton().camera_attributes_create(),
        ))
    }
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
