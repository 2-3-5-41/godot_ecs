use godot::{
    engine::{rendering_server::SkyMode, Image, RenderingServer},
    prelude::{Rid, Vector2i},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::material::Material;

resource_object!(Sky, sky_create, RenderingServer);

impl Sky {
    pub fn bake_panorama(
        &self,
        energy: f32,
        bake_irradiance: bool,
        size: Vector2i,
    ) -> Option<godot::prelude::Gd<Image>> {
        RenderingServer::singleton().sky_bake_panorama(
            self.get_rid(),
            energy,
            bake_irradiance,
            size,
        )
    }
    pub fn set_material(&self, material: Material) -> &Self {
        RenderingServer::singleton().sky_set_material(self.get_rid(), material.get_rid());
        self
    }
    pub fn set_mode(&self, mode: SkyMode) -> &Self {
        RenderingServer::singleton().sky_set_mode(self.get_rid(), mode);
        self
    }
    pub fn set_radiance_size(&self, radiance_size: i32) -> &Self {
        RenderingServer::singleton().sky_set_radiance_size(self.get_rid(), radiance_size);
        self
    }
}
