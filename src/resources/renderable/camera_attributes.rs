use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{
    engine::{
        rendering_server::{DOFBlurQuality, DOFBokehShape},
        RenderingServer,
    },
    prelude::Rid,
};

resource_object!(CameraAttributes, camera_attributes_create, RenderingServer);

impl CameraAttributes {
    pub fn set_auto_exposer(
        &self,
        enable: bool,
        min_sensitivity: f32,
        max_sensitivity: f32,
        speed: f32,
        scale: f32,
    ) -> &Self {
        RenderingServer::singleton().camera_attributes_set_auto_exposure(
            self.get_rid(),
            enable,
            min_sensitivity,
            max_sensitivity,
            speed,
            scale,
        );
        &self
    }
    pub fn set_dof_blur(
        &self,
        far_enable: bool,
        far_distance: f32,
        far_transition: f32,
        near_enable: bool,
        near_distance: f32,
        near_transition: f32,
        amount: f32,
    ) -> &Self {
        RenderingServer::singleton().camera_attributes_set_dof_blur(
            self.get_rid(),
            far_enable,
            far_distance,
            far_transition,
            near_enable,
            near_distance,
            near_transition,
            amount,
        );
        &self
    }
    pub fn set_dof_blur_bokeh_shape(&self, shape: DOFBokehShape) -> &Self {
        RenderingServer::singleton().camera_attributes_set_dof_blur_bokeh_shape(shape);
        &self
    }
    pub fn set_dof_blue_quality(&self, quality: DOFBlurQuality, use_jitter: bool) -> &Self {
        RenderingServer::singleton().camera_attributes_set_dof_blur_quality(quality, use_jitter);
        &self
    }
    pub fn set_exposure(&self, multiplier: f32, normalization: f32) -> &Self {
        RenderingServer::singleton().camera_attributes_set_exposure(
            self.get_rid(),
            multiplier,
            normalization,
        );
        &self
    }
}
