use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(DirectionalLight, directional_light_create);

// TODO: Provide a builder API for `DirectionalLight`
impl DirectionalLight {}
