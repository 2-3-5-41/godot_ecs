use bevy_ecs::prelude::Component;
use godot::builtin::Rid;
use godot::engine::RenderingServer;

#[derive(Component, Copy, Clone, Debug)]
pub struct CameraAttributes(Rid);

impl CameraAttributes {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().camera_attributes_create())
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
    pub fn set_auto_exposure(
        self,
        enable: bool,
        min_sensitivity: f32,
        max_sensitivity: f32,
        speed: f32,
        scale: f32,
    ) -> Self {
        RenderingServer::singleton().camera_attributes_set_auto_exposure(
            self.0,
            enable,
            min_sensitivity,
            max_sensitivity,
            speed,
            scale,
        );
        self
    }
    pub fn set_dof_blur(
        self,
        far_enable: bool,
        far_distance: f32,
        far_transition: f32,
        near_enable: bool,
        near_distance: f32,
        near_transition: f32,
        amount: f32,
    ) -> Self {
        RenderingServer::singleton().camera_attributes_set_dof_blur(
            self.0,
            far_enable,
            far_distance,
            far_transition,
            near_enable,
            near_distance,
            near_transition,
            amount,
        );
        self
    }
    pub fn set_exposure(self, multiplier: f32, normalization: f32) -> Self {
        RenderingServer::singleton().camera_attributes_set_exposure(
            self.0,
            multiplier,
            normalization,
        );
        self
    }
}

impl Default for CameraAttributes {
    fn default() -> Self {
        Self::new()
    }
}
