use godot::{
    engine::{
        rendering_server::{LightDirectionalShadowMode, LightDirectionalSkyMode},
        RenderingServer,
    },
    prelude::Rid,
};

use crate::resources::{
    traits::{CommonLight3D, ResourceId},
    utils::macros::resource_object,
};

resource_object!(DirectionalLight, directional_light_create, RenderingServer);

impl CommonLight3D for DirectionalLight {}

impl DirectionalLight {
    pub fn set_blend_splits(&self, enable: bool) -> &Self {
        RenderingServer::singleton().light_directional_set_blend_splits(self.get_rid(), enable);
        self
    }
    pub fn set_shadow_mode(&self, mode: LightDirectionalShadowMode) -> &Self {
        RenderingServer::singleton().light_directional_set_shadow_mode(self.get_rid(), mode);
        self
    }
    pub fn set_sky_mode(&self, mode: LightDirectionalSkyMode) -> &Self {
        RenderingServer::singleton().light_directional_set_sky_mode(self.get_rid(), mode);
        self
    }
}
