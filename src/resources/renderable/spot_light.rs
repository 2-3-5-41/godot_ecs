use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(SpotLight, spot_light_create);

// TODO: Provide a builder API for `SpotLight`
impl SpotLight {}
