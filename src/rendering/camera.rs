use bevy_ecs::component::Component;
use godot::{
    engine::RenderingServer,
    prelude::{Rid, Transform3D, Vector2},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Camera(Rid);

impl Camera {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().camera_create())
    }
    pub fn get_rid(&self) -> Rid {
        self.0.clone()
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn free(&self) {
        RenderingServer::singleton().free_rid(self.0);
    }
    pub fn set_attributes(self, effects: Rid) -> Self {
        RenderingServer::singleton().camera_set_camera_attributes(self.0, effects);
        self
    }
    pub fn set_cull_mask(self, layers: u32) -> Self {
        RenderingServer::singleton().camera_set_cull_mask(self.0, layers);
        self
    }
    pub fn set_environment(self, env: Rid) -> Self {
        RenderingServer::singleton().camera_set_environment(self.0, env);
        self
    }
    pub fn set_frustum(self, size: f32, offset: Vector2, z_near: f32, z_far: f32) -> Self {
        RenderingServer::singleton().camera_set_frustum(self.0, size, offset, z_near, z_far);
        self
    }
    pub fn set_orthogonal(self, size: f32, z_near: f32, z_far: f32) -> Self {
        RenderingServer::singleton().camera_set_orthogonal(self.0, size, z_near, z_far);
        self
    }
    pub fn set_perspective(self, fovy_degrees: f32, z_near: f32, z_far: f32) -> Self {
        RenderingServer::singleton().camera_set_perspective(self.0, fovy_degrees, z_near, z_far);
        self
    }
    pub fn set_transform(self, transform: Transform3D) -> Self {
        RenderingServer::singleton().camera_set_transform(self.0, transform);
        self
    }
    pub fn use_vertical_aspect(self, enable: bool) -> Self {
        RenderingServer::singleton().camera_set_use_vertical_aspect(self.0, enable);
        self
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
