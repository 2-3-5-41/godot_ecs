use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(VoxelGI, voxel_gi_create, RenderingServer);

// TODO: Provide a builder API for `VoxelGI`
impl VoxelGI {}
