use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(VoxelGI, voxel_gi_create);

// TODO: Provide a builder API for `VoxelGI`
impl VoxelGI {}
