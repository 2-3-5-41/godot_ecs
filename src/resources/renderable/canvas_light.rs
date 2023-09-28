use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(CanvasLight, canvas_light_create);

// TODO: Provide a builder API for `CanvasLight`
impl CanvasLight {}
