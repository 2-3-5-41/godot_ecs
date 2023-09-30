use godot::{
    engine::{rendering_server::LightOmniShadowMode, RenderingServer},
    prelude::Rid,
};

use crate::resources::{
    traits::{CommonLight3D, ResourceId},
    utils::macros::resource_object,
};

resource_object!(OmniLight, omni_light_create, RenderingServer);

impl CommonLight3D for OmniLight {}

impl OmniLight {
    pub fn set_shadow_mode(&self, mode: LightOmniShadowMode) -> &Self {
        RenderingServer::singleton().light_omni_set_shadow_mode(self.get_rid(), mode);
        self
    }
}
