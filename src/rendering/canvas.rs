use bevy_ecs::prelude::Component;
use godot::engine::RenderingServer;
use godot::prelude::{Rid, Vector2, Color};

#[derive(Component, Copy, Clone, Debug)]
pub struct Canvas(Rid);

impl Canvas {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().canvas_create())
    }
    pub fn get_rid(&self) -> Rid {
        self.0.clone()
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn free(&self) {
        RenderingServer::singleton().free_rid(self.0);
    }
    pub fn set_item_mirroring(self, item: Rid, mirroring: Vector2) -> Self {
        RenderingServer::singleton().canvas_set_item_mirroring(self.0, item, mirroring);
        self
    }
    pub fn set_modulate(self, color: Color) -> Self {
        RenderingServer::singleton().canvas_set_modulate(self.0, color);
        self
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
