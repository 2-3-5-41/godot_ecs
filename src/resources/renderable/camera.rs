use godot::{
    engine::RenderingServer,
    prelude::{Rid, Transform3D, Vector2},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{camera_attributes::CameraAttributes, environment::Environment};

resource_object!(Camera, camera_create, RenderingServer);

impl Camera {
    pub fn set_camera_attributes(&self, effects: CameraAttributes) -> &Self {
        RenderingServer::singleton()
            .camera_set_camera_attributes(self.get_rid(), effects.get_rid());
        &self
    }
    pub fn set_cull_mask(&self, layers: u32) -> &Self {
        RenderingServer::singleton().camera_set_cull_mask(self.get_rid(), layers);
        &self
    }
    pub fn set_environment(&self, env: Environment) -> &Self {
        RenderingServer::singleton().camera_set_environment(self.get_rid(), env.get_rid());
        &self
    }
    pub fn set_frustum(
        &self,
        size: f32,
        offset: Vector2,
        z_near: f32,
        z_far: f32,
    ) -> &Self {
        RenderingServer::singleton().camera_set_frustum(
            self.get_rid(),
            size,
            offset,
            z_near,
            z_far,
        );
        &self
    }
    pub fn set_orthographic(&self, size: f32, z_near: f32, z_far: f32) -> &Self {
        RenderingServer::singleton().camera_set_orthogonal(self.get_rid(), size, z_near, z_far);
        &self
    }
    pub fn set_perspective(&self, fovy_degrees: f32, z_near: f32, z_far: f32) -> &Self {
        RenderingServer::singleton().camera_set_perspective(
            self.get_rid(),
            fovy_degrees,
            z_near,
            z_far,
        );
        &self
    }
    pub fn set_transform(&self, transform: Transform3D) -> &Self {
        RenderingServer::singleton().camera_set_transform(self.get_rid(), transform);
        &self
    }
    pub fn set_use_vertical_aspect(&self, enable: bool) -> &Self {
        RenderingServer::singleton().camera_set_use_vertical_aspect(self.get_rid(), enable);
        &self
    }
}
