use godot::{
    engine::RenderingServer,
    prelude::{Rid, Transform2D},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{canvas::Canvas, canvas_occluder_polygon::CanvasOccluderPolygon};

resource_object!(
    CanvasLightOccluder,
    canvas_light_occluder_create,
    RenderingServer
);

impl CanvasLightOccluder {
    pub fn attach_to_canvas(&self, canvas: Canvas) -> &Self {
        RenderingServer::singleton()
            .canvas_light_occluder_attach_to_canvas(self.get_rid(), canvas.get_rid());
        self
    }
    pub fn set_as_sdf_collision(&self, enable: bool) -> &Self {
        RenderingServer::singleton()
            .canvas_light_occluder_set_as_sdf_collision(self.get_rid(), enable);
        self
    }
    pub fn set_enabled(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().canvas_light_occluder_set_enabled(self.get_rid(), enabled);
        self
    }
    pub fn set_light_mask(&self, mask: i32) -> &Self {
        RenderingServer::singleton().canvas_light_occluder_set_light_mask(self.get_rid(), mask);
        self
    }
    pub fn set_polygon(&self, polygon: CanvasOccluderPolygon) -> &Self {
        RenderingServer::singleton()
            .canvas_light_occluder_set_polygon(self.get_rid(), polygon.get_rid());
        self
    }
    pub fn set_transform(&self, transform: Transform2D) -> &Self {
        RenderingServer::singleton().canvas_light_occluder_set_transform(self.get_rid(), transform);
        self
    }
}
