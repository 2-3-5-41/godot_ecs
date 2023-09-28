use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(CanvasItem, canvas_item_create);

// TODO: Provide a builder API for `CanvasItem`
impl CanvasItem {}
