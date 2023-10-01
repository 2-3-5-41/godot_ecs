use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{camera::Camera, canvas::Canvas};

resource_object!(Viewport, viewport_create, RenderingServer);

impl Viewport {
    pub fn attach_camera(&self, camera: &Camera) -> &Self {
        RenderingServer::singleton().viewport_attach_camera(self.get_rid(), camera.get_rid());
        self
    }
    pub fn attach_canvas(&self, canvas: &Canvas) -> &Self {
        RenderingServer::singleton().viewport_attach_canvas(self.get_rid(), canvas.get_rid());
        self
    }
}
