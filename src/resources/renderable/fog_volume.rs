use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(FogVolume, fog_volume_create, RenderingServer);

// TODO: Provide a builder API for `FogVolume`
impl FogVolume {}
