use crate::{
    math::transform::GdTransform3D,
    rendering::{resource::*, Renderable},
};
use bevy_ecs::{bundle::Bundle, component::Component};

use super::viewport::Viewport;

#[derive(Component, Clone, Copy, Debug)]
pub struct Camera {
    resource: RendererResource,
}

impl Renderable for Camera {
    fn new() -> Self {
        Self {
            resource: RendererResource::create(RendererCreate::Camera),
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

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Bundle, Default, Clone, Copy, Debug)]
pub struct CameraBundle {
    pub camera: Camera,
    pub viewport: Viewport,
    pub transform_3d: GdTransform3D,
}

impl CameraBundle {
    pub fn with_viewport(viewport: Viewport) -> Self {
        Self {
            camera: Camera::default(),
            viewport,
            transform_3d: GdTransform3D::default(),
        }
    }
}
