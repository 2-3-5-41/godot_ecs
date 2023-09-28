use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(CanvasLightOccluder, canvas_light_occluder_create);

// TODO: Provide a builder API for `CanvasLightOccluder`
impl CanvasLightOccluder {}
