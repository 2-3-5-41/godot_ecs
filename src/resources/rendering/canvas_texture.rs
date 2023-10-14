use godot::{
    engine::{
        rendering_server::{
            CanvasItemTextureFilter, CanvasItemTextureRepeat, CanvasTextureChannel,
        },
        RenderingServer,
    },
    prelude::{Color, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(CanvasTexture, canvas_texture_create, RenderingServer);

impl CanvasTexture {
    pub fn set_channel(&self, channel: CanvasTextureChannel, texture: Rid) -> &Self {
        RenderingServer::singleton().canvas_texture_set_channel(self.get_rid(), channel, texture);
        self
    }
    pub fn set_shading_parameters(&self, base_color: Color, shininess: f32) -> &Self {
        RenderingServer::singleton().canvas_texture_set_shading_parameters(
            self.get_rid(),
            base_color,
            shininess,
        );
        self
    }
    pub fn set_texture_filter(&self, filter: CanvasItemTextureFilter) -> &Self {
        RenderingServer::singleton().canvas_texture_set_texture_filter(self.get_rid(), filter);
        self
    }
    pub fn set_texture_repeat(&self, repeat: CanvasItemTextureRepeat) -> &Self {
        RenderingServer::singleton().canvas_texture_set_texture_repeat(self.get_rid(), repeat);
        self
    }
}
