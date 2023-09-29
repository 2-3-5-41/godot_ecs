use godot::{
    engine::{
        rendering_server::{CanvasLightBlendMode, CanvasLightMode, CanvasLightShadowFilter},
        RenderingServer,
    },
    prelude::{Color, Rid, Transform2D, Vector2},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::canvas::Canvas;

resource_object!(CanvasLight, canvas_light_create, RenderingServer);

impl CanvasLight {
    pub fn attatch_to_canvas(&self, canvas: Canvas) -> &Self {
        RenderingServer::singleton()
            .canvas_light_attach_to_canvas(self.get_rid(), canvas.get_rid());
        self
    }
    pub fn set_blend_mode(&self, mode: CanvasLightBlendMode) -> &Self {
        RenderingServer::singleton().canvas_light_set_blend_mode(self.get_rid(), mode);
        self
    }
    pub fn set_color(&self, color: Color) -> &Self {
        RenderingServer::singleton().canvas_light_set_color(self.get_rid(), color);
        self
    }
    pub fn set_enabled(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().canvas_light_set_enabled(self.get_rid(), enabled);
        self
    }
    pub fn set_energy(&self, energy: f32) -> &Self {
        RenderingServer::singleton().canvas_light_set_energy(self.get_rid(), energy);
        self
    }
    pub fn set_height(&self, height: f32) -> &Self {
        RenderingServer::singleton().canvas_light_set_height(self.get_rid(), height);
        self
    }
    pub fn set_item_cull_mask(&self, mask: i32) -> &Self {
        RenderingServer::singleton().canvas_light_set_item_cull_mask(self.get_rid(), mask);
        self
    }
    pub fn set_item_shadow_cull_mask(&self, mask: i32) -> &Self {
        RenderingServer::singleton().canvas_light_set_item_shadow_cull_mask(self.get_rid(), mask);
        self
    }
    pub fn set_layer_range(&self, min_layer: i32, max_layer: i32) -> &Self {
        RenderingServer::singleton().canvas_light_set_layer_range(
            self.get_rid(),
            min_layer,
            max_layer,
        );
        self
    }
    pub fn set_mode(&self, mode: CanvasLightMode) -> &Self {
        RenderingServer::singleton().canvas_light_set_mode(self.get_rid(), mode);
        self
    }
    pub fn set_shadow_color(&self, color: Color) -> &Self {
        RenderingServer::singleton().canvas_light_set_shadow_color(self.get_rid(), color);
        self
    }
    pub fn set_shadow_enabled(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().canvas_light_set_shadow_enabled(self.get_rid(), enabled);
        self
    }
    pub fn set_shadow_filter(&self, filter: CanvasLightShadowFilter) -> &Self {
        RenderingServer::singleton().canvas_light_set_shadow_filter(self.get_rid(), filter);
        self
    }
    pub fn set_shadow_smooth(&self, smooth: f32) -> &Self {
        RenderingServer::singleton().canvas_light_set_shadow_smooth(self.get_rid(), smooth);
        self
    }
    pub fn set_texture(&self, texture: Rid) -> &Self {
        RenderingServer::singleton().canvas_light_set_texture(self.get_rid(), texture);
        self
    }
    pub fn set_texture_offset(&self, offset: Vector2) -> &Self {
        RenderingServer::singleton().canvas_light_set_texture_offset(self.get_rid(), offset);
        self
    }
    pub fn set_texture_scale(&self, scale: f32) -> &Self {
        RenderingServer::singleton().canvas_light_set_texture_scale(self.get_rid(), scale);
        self
    }
    pub fn set_transform(&self, transform: Transform2D) -> &Self {
        RenderingServer::singleton().canvas_light_set_transform(self.get_rid(), transform);
        self
    }
    pub fn set_z_range(&self, min_z: i32, max_z: i32) -> &Self {
        RenderingServer::singleton().canvas_light_set_z_range(self.get_rid(), min_z, max_z);
        self
    }
}
