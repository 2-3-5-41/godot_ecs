use godot::{
    engine::{rendering_server::FogVolumeShape, RenderingServer},
    prelude::{Rid, Vector3},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::material::Material;

resource_object!(FogVolume, fog_volume_create, RenderingServer);

impl FogVolume {
    pub fn set_material(&self, material: Material) -> &Self {
        RenderingServer::singleton().fog_volume_set_material(self.get_rid(), material.get_rid());
        self
    }
    pub fn set_shape(&self, shape: FogVolumeShape) -> &Self {
        RenderingServer::singleton().fog_volume_set_shape(self.get_rid(), shape);
        self
    }
    pub fn set_size(&self, size: Vector3) -> &Self {
        RenderingServer::singleton().fog_volume_set_size(self.get_rid(), size);
        self
    }
}
