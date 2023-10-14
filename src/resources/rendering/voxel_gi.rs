use godot::{
    engine::{rendering_server::VoxelGIQuality, RenderingServer},
    prelude::{Aabb, PackedByteArray, PackedInt32Array, Rid, Transform3D, Vector3i},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(VoxelGI, voxel_gi_create, RenderingServer);

impl VoxelGI {
    pub fn allocate_data(
        &self,
        to_cell_xform: Transform3D,
        aabb: Aabb,
        octree_size: Vector3i,
        octree_cells: PackedByteArray,
        data_cells: PackedByteArray,
        distance_field: PackedByteArray,
        level_counts: PackedInt32Array,
    ) -> &Self {
        RenderingServer::singleton().voxel_gi_allocate_data(
            self.get_rid(),
            to_cell_xform,
            aabb,
            octree_size,
            octree_cells,
            data_cells,
            distance_field,
            level_counts,
        );
        self
    }
    pub fn get_data_cells(&self) -> PackedByteArray {
        RenderingServer::singleton().voxel_gi_get_data_cells(self.get_rid())
    }
    pub fn get_distance_field(&self) -> PackedByteArray {
        RenderingServer::singleton().voxel_gi_get_distance_field(self.get_rid())
    }
    pub fn get_level_counts(&self) -> PackedInt32Array {
        RenderingServer::singleton().voxel_gi_get_level_counts(self.get_rid())
    }
    pub fn get_octree_cell(&self) -> PackedByteArray {
        RenderingServer::singleton().voxel_gi_get_octree_cells(self.get_rid())
    }
    pub fn get_octree_size(&self) -> Vector3i {
        RenderingServer::singleton().voxel_gi_get_octree_size(self.get_rid())
    }
    pub fn get_to_cell_xform(&self) -> Transform3D {
        RenderingServer::singleton().voxel_gi_get_to_cell_xform(self.get_rid())
    }
    pub fn set_baked_exposure_normalization(&self, baked_exposure: f32) -> &Self {
        RenderingServer::singleton()
            .voxel_gi_set_baked_exposure_normalization(self.get_rid(), baked_exposure);
        self
    }
    pub fn set_bias(&self, bias: f32) -> &Self {
        RenderingServer::singleton().voxel_gi_set_bias(self.get_rid(), bias);
        self
    }
    pub fn set_dynamic_range(&self, range: f32) -> &Self {
        RenderingServer::singleton().voxel_gi_set_dynamic_range(self.get_rid(), range);
        self
    }
    pub fn set_energy(&self, energy: f32) -> &Self {
        RenderingServer::singleton().voxel_gi_set_energy(self.get_rid(), energy);
        self
    }
    pub fn set_interior(&self, enable: bool) -> &Self {
        RenderingServer::singleton().voxel_gi_set_interior(self.get_rid(), enable);
        self
    }
    pub fn set_normal_bias(&self, bias: f32) -> &Self {
        RenderingServer::singleton().voxel_gi_set_normal_bias(self.get_rid(), bias);
        self
    }
    pub fn set_propagation(&self, amount: f32) -> &Self {
        RenderingServer::singleton().voxel_gi_set_propagation(self.get_rid(), amount);
        self
    }
    pub fn set_quality(quality: VoxelGIQuality) {
        RenderingServer::singleton().voxel_gi_set_quality(quality);
    }
    pub fn set_use_two_bounces(&self, enable: bool) -> &Self {
        RenderingServer::singleton().voxel_gi_set_use_two_bounces(self.get_rid(), enable);
        self
    }
}
