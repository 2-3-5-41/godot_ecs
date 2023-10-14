use godot::{
    engine::{rendering_server::CanvasOccluderPolygonCullMode, RenderingServer},
    prelude::{PackedVector2Array, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    CanvasOccluderPolygon,
    canvas_occluder_polygon_create,
    RenderingServer
);

impl CanvasOccluderPolygon {
    pub fn set_cull_mode(&self, mode: CanvasOccluderPolygonCullMode) -> &Self {
        RenderingServer::singleton().canvas_occluder_polygon_set_cull_mode(self.get_rid(), mode);
        self
    }
    pub fn set_shape(&self, shape: PackedVector2Array, closed: bool) -> &Self {
        RenderingServer::singleton().canvas_occluder_polygon_set_shape(
            self.get_rid(),
            shape,
            closed,
        );
        self
    }
}
