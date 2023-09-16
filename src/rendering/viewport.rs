use bevy_ecs::component::Component;
use godot::{engine::RenderingServer, prelude::Rid};

#[derive(Component, Debug, Clone, Copy)]
pub struct Viewport(Rid);

impl Viewport {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().viewport_create())
    }

    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }

    pub fn free(&self) {
        RenderingServer::singleton().free_rid(self.0);
    }

    pub fn attach_camera(self, camera: Rid) -> Self {
        RenderingServer::singleton().viewport_attach_camera(self.0, camera);
        self
    }

    pub fn attach_canvas(self, canvas: Rid) -> Self {
        RenderingServer::singleton().viewport_attach_canvas(self.0, canvas);
        self
    }

    pub fn remove_canvas(self, canvas: Rid) -> Self {
        RenderingServer::singleton().viewport_remove_canvas(self.0, canvas);
        self
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
