use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    CanvasOccluderPolygon,
    canvas_occluder_polygon_create,
    RenderingServer
);

// TODO: Provide a builder API for `CanvasOccluderPolygon`
impl CanvasOccluderPolygon {}
