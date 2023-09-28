use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(FogVolume, fog_volume_create);

// TODO: Provide a builder API for `FogVolume`
impl FogVolume {}
