use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Canvas, canvas_create);

// TODO: Provide a builder API for `Canvas`
impl Canvas {}
