use bevy_ecs::component::Component;
use godot::{
    engine::{
        rendering_server::{CanvasLightBlendMode, CanvasLightMode, CanvasLightShadowFilter},
        RenderingServer,
    },
    prelude::{Color, Rid, Transform2D, Vector2},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct CanvasLight(Rid);

impl CanvasLight {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().canvas_light_create())
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn get_rid(&self) -> Rid {
        self.0.clone()
    }
    pub fn free(&self) {
        RenderingServer::singleton().free_rid(self.0);
    }
    pub fn attach_to_canvas(self, canvas: Rid) -> Self {
        RenderingServer::singleton().canvas_light_attach_to_canvas(self.0, canvas);
        self
    }
    pub fn set_blend_mode(self, mode: CanvasLightBlendMode) -> Self {
        RenderingServer::singleton().canvas_light_set_blend_mode(self.0, mode);
        self
    }
    pub fn set_color(self, color: Color) -> Self {
        RenderingServer::singleton().canvas_light_set_color(self.0, color);
        self
    }
    pub fn set_enabled(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_light_set_enabled(self.0, enabled);
        self
    }
    pub fn set_energy(self, energy: f32) -> Self {
        RenderingServer::singleton().canvas_light_set_energy(self.0, energy);
        self
    }
    pub fn set_height(self, height: f32) -> Self {
        RenderingServer::singleton().canvas_light_set_height(self.0, height);
        self
    }
    pub fn set_item_cull_mask(self, mask: i32) -> Self {
        RenderingServer::singleton().canvas_light_set_item_cull_mask(self.0, mask);
        self
    }
    pub fn set_item_shadow_cull_mask(self, mask: i32) -> Self {
        RenderingServer::singleton().canvas_light_set_item_shadow_cull_mask(self.0, mask);
        self
    }
    pub fn set_layer_rang(self, min_layer: i32, max_layer: i32) -> Self {
        RenderingServer::singleton().canvas_light_set_layer_range(self.0, min_layer, max_layer);
        self
    }
    pub fn set_mode(self, mode: CanvasLightMode) -> Self {
        RenderingServer::singleton().canvas_light_set_mode(self.0, mode);
        self
    }
    pub fn set_shadow_color(self, color: Color) -> Self {
        RenderingServer::singleton().canvas_light_set_shadow_color(self.0, color);
        self
    }
    pub fn set_shadow_enabled(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_light_set_shadow_enabled(self.0, enabled);
        self
    }
    pub fn set_shadow_filter(self, filter: CanvasLightShadowFilter) -> Self {
        RenderingServer::singleton().canvas_light_set_shadow_filter(self.0, filter);
        self
    }
    pub fn set_shadow_smooth(self, smooth: f32) -> Self {
        RenderingServer::singleton().canvas_light_set_shadow_smooth(self.0, smooth);
        self
    }
    pub fn set_texture(self, texture: Rid) -> Self {
        RenderingServer::singleton().canvas_light_set_texture(self.0, texture);
        self
    }
    pub fn set_texture_offset(self, offset: Vector2) -> Self {
        RenderingServer::singleton().canvas_light_set_texture_offset(self.0, offset);
        self
    }
    pub fn set_texture_scale(self, scale: f32) -> Self {
        RenderingServer::singleton().canvas_light_set_texture_scale(self.0, scale);
        self
    }
    pub fn set_transform(self, transform: Transform2D) -> Self {
        RenderingServer::singleton().canvas_light_set_transform(self.0, transform);
        self
    }
    pub fn set_z_range(self, min_z: i32, max_z: i32) -> Self {
        RenderingServer::singleton().canvas_light_set_z_range(self.0, min_z, max_z);
        self
    }
}

impl Default for CanvasLight {
    fn default() -> Self {
        Self::new()
    }
}
