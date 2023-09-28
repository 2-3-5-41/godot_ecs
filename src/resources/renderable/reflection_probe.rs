use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(ReflectionProbe, reflection_probe_create);

// TODO: Provide a builder API for `ReflectionProbe`
impl ReflectionProbe {}
