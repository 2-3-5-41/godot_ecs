use godot::{
    engine::RenderingServer,
    prelude::{Aabb, PackedColorArray, PackedInt32Array, PackedVector3Array, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Lightmap, lightmap_create, RenderingServer);

impl Lightmap {
    pub fn get_probe_capture_bsp_tree(&self) -> PackedInt32Array {
        RenderingServer::singleton().lightmap_get_probe_capture_bsp_tree(self.get_rid())
    }
    pub fn get_probe_capture_points(&self) -> PackedVector3Array {
        RenderingServer::singleton().lightmap_get_probe_capture_points(self.get_rid())
    }
    pub fn get_probe_capture_sh(&self) -> PackedColorArray {
        RenderingServer::singleton().lightmap_get_probe_capture_sh(self.get_rid())
    }
    pub fn get_probe_capture_tetrahedra(&self) -> PackedInt32Array {
        RenderingServer::singleton().lightmap_get_probe_capture_tetrahedra(self.get_rid())
    }
    pub fn set_backed_exposure_normalized(&self, baked_exposure: f32) -> &Self {
        RenderingServer::singleton()
            .lightmap_set_baked_exposure_normalization(self.get_rid(), baked_exposure);
        self
    }
    pub fn set_probe_bounds(&self, bounds: Aabb) -> &Self {
        RenderingServer::singleton().lightmap_set_probe_bounds(self.get_rid(), bounds);
        self
    }
    pub fn set_probe_capture_data(
        &self,
        points: PackedVector3Array,
        point_sh: PackedColorArray,
        tetrahedra: PackedInt32Array,
        bsp_tree: PackedInt32Array,
    ) -> &Self {
        RenderingServer::singleton().lightmap_set_probe_capture_data(
            self.get_rid(),
            points,
            point_sh,
            tetrahedra,
            bsp_tree,
        );
        self
    }
    pub fn set_probe_capture_update_speed(speed: f32) {
        RenderingServer::singleton().lightmap_set_probe_capture_update_speed(speed);
    }
    pub fn set_probe_interior(&self, interior: bool) -> &Self {
        RenderingServer::singleton().lightmap_set_probe_interior(self.get_rid(), interior);
        self
    }
    pub fn set_textures(&self, light: Rid, uses_sh: bool) -> &Self {
        RenderingServer::singleton().lightmap_set_textures(self.get_rid(), light, uses_sh);
        self
    }
}
