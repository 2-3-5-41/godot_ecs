use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(CanvasOccluderPolygon, canvas_occluder_polygon_create);

// TODO: Provide a builder API for `CanvasOccluderPolygon`
impl CanvasOccluderPolygon {}
