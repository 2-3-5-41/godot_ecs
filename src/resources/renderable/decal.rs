use godot::{
    engine::{rendering_server::DecalTexture, RenderingServer},
    prelude::{Color, Rid, Vector3},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Decal, decal_create, RenderingServer);

impl Decal {
    pub fn set_albedo_mix(&self, albedo_mix: f32) -> &Self {
        RenderingServer::singleton().decal_set_albedo_mix(self.get_rid(), albedo_mix);
        self
    }
    pub fn set_cull_mask(&self, mask: u32) -> &Self {
        RenderingServer::singleton().decal_set_cull_mask(self.get_rid(), mask);
        self
    }
    pub fn set_distance_fade(&self, enabled: bool, begin: f32, length: f32) -> &Self {
        RenderingServer::singleton().decal_set_distance_fade(
            self.get_rid(),
            enabled,
            begin,
            length,
        );
        self
    }
    pub fn set_emission_energy(&self, energy: f32) -> &Self {
        RenderingServer::singleton().decal_set_emission_energy(self.get_rid(), energy);
        self
    }
    pub fn set_fade(&self, above: f32, below: f32) -> &Self {
        RenderingServer::singleton().decal_set_fade(self.get_rid(), above, below);
        self
    }
    pub fn set_modulate(&self, color: Color) -> &Self {
        RenderingServer::singleton().decal_set_modulate(self.get_rid(), color);
        self
    }
    pub fn set_normal_fade(&self, fade: f32) -> &Self {
        RenderingServer::singleton().decal_set_normal_fade(self.get_rid(), fade);
        self
    }
    pub fn set_size(&self, size: Vector3) -> &Self {
        RenderingServer::singleton().decal_set_size(self.get_rid(), size);
        self
    }
    pub fn set_texture(&self, decal_type: DecalTexture, texture: Rid) -> &Self {
        RenderingServer::singleton().decal_set_texture(self.get_rid(), decal_type, texture);
        self
    }
}
