use bevy_ecs::prelude::Component;
use godot::{
    builtin::{Color, Rect2, Rid, Vector2},
    engine::{
        rendering_server::{
            CanvasGroupMode, CanvasItemTextureFilter, CanvasItemTextureRepeat, NinePatchAxisMode,
        },
        RenderingServer,
    },
    prelude::{
        Callable, PackedColorArray, PackedFloat32Array, PackedInt32Array, PackedVector2Array,
        Transform2D,
    },
};

#[derive(Component, Copy, Clone, Debug)]
pub struct CanvasItem(Rid);

impl CanvasItem {
    pub fn new() -> Self {
        Self(RenderingServer::singleton().canvas_item_create())
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
    pub fn add_animation_slice(
        self,
        animation_length: f64,
        slice_begin: f64,
        slice_end: f64,
        offset: f64,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_animation_slice_ex(self.0, animation_length, slice_begin, slice_end)
            .offset(offset)
            .done();
        self
    }
    pub fn add_circle(self, pos: Vector2, radius: f32, color: Color) -> Self {
        RenderingServer::singleton().canvas_item_add_circle(self.0, pos, radius, color);
        self
    }
    pub fn add_clip_ignore(self, ignore: bool) -> Self {
        RenderingServer::singleton().canvas_item_add_clip_ignore(self.0, ignore);
        self
    }
    pub fn add_lcd_texture_rect_region(
        self,
        rect: Rect2,
        texture: Rid,
        src_rect: Rect2,
        modulate: Color,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_lcd_texture_rect_region(self.0, rect, texture, src_rect, modulate);
        self
    }
    pub fn add_line(
        self,
        from: Vector2,
        to: Vector2,
        color: Color,
        width: f32,
        antialiased: bool,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_line_ex(self.0, from, to, color)
            .width(width)
            .antialiased(antialiased)
            .done();
        self
    }
    pub fn add_mesh(
        self,
        mesh: Rid,
        transform: Transform2D,
        modulate: Color,
        texture: Rid,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_mesh_ex(self.0, mesh)
            .transform(transform)
            .modulate(modulate)
            .texture(texture)
            .done();
        self
    }
    pub fn add_msdf_texture_rect_region(
        self,
        rect: Rect2,
        texture: Rid,
        src_rect: Rect2,
        modulate: Color,
        outline_size: i32,
        px_range: f32,
        scale: f32,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_msdf_texture_rect_region_ex(self.0, rect, texture, src_rect)
            .modulate(modulate)
            .outline_size(outline_size)
            .px_range(px_range)
            .scale(scale)
            .done();
        self
    }
    pub fn add_multiline(
        self,
        points: PackedVector2Array,
        colors: PackedColorArray,
        width: f32,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_multiline_ex(self.0, points, colors)
            .width(width)
            .done();
        self
    }
    pub fn add_multimesh(self, mesh: Rid, texture: Rid) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_multimesh_ex(self.0, mesh)
            .texture(texture)
            .done();
        self
    }
    pub fn add_nine_patch(
        self,
        rect: Rect2,
        source: Rect2,
        texture: Rid,
        topleft: Vector2,
        bottomright: Vector2,
        x_axis_mode: NinePatchAxisMode,
        y_axis_mode: NinePatchAxisMode,
        draw_center: bool,
        modulate: Color,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_nine_patch_ex(self.0, rect, source, texture, topleft, bottomright)
            .x_axis_mode(x_axis_mode)
            .y_axis_mode(y_axis_mode)
            .draw_center(draw_center)
            .modulate(modulate)
            .done();
        self
    }
    pub fn add_particles(self, particles: Rid, texture: Rid) -> Self {
        RenderingServer::singleton().canvas_item_add_particles(self.0, particles, texture);
        self
    }
    pub fn add_polygon(
        self,
        points: PackedVector2Array,
        colors: PackedColorArray,
        uvs: PackedVector2Array,
        texture: Rid,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_polygon_ex(self.0, points, colors)
            .uvs(uvs)
            .texture(texture)
            .done();
        self
    }
    pub fn add_polyline(
        self,
        points: PackedVector2Array,
        colors: PackedColorArray,
        width: f32,
        antialiased: bool,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_polyline_ex(self.0, points, colors)
            .width(width)
            .antialiased(antialiased)
            .done();
        self
    }
    pub fn add_primitive(
        self,
        points: PackedVector2Array,
        colors: PackedColorArray,
        uvs: PackedVector2Array,
        texture: Rid,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_primitive(self.0, points, colors, uvs, texture);
        self
    }
    pub fn add_rect(self, rect: Rect2, color: Color) -> Self {
        RenderingServer::singleton().canvas_item_add_rect(self.0, rect, color);
        self
    }
    pub fn add_set_transform(self, transform: Transform2D) -> Self {
        RenderingServer::singleton().canvas_item_add_set_transform(self.0, transform);
        self
    }
    pub fn add_texture_rect(
        self,
        rect: Rect2,
        texture: Rid,
        tile: bool,
        modulate: Color,
        transpose: bool,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_texture_rect_ex(self.0, rect, texture)
            .tile(tile)
            .modulate(modulate)
            .transpose(transpose)
            .done();
        self
    }
    pub fn add_texture_rect_region(
        self,
        rect: Rect2,
        texture: Rid,
        src_rect: Rect2,
        modulate: Color,
        transpose: bool,
        clip_uv: bool,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_texture_rect_region_ex(self.0, rect, texture, src_rect)
            .modulate(modulate)
            .transpose(transpose)
            .clip_uv(clip_uv)
            .done();
        self
    }
    pub fn add_triangle_array(
        self,
        indices: PackedInt32Array,
        points: PackedVector2Array,
        colors: PackedColorArray,
        uvs: PackedVector2Array,
        bones: PackedInt32Array,
        weights: PackedFloat32Array,
        texture: Rid,
        count: i32,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_add_triangle_array_ex(self.0, indices, points, colors)
            .uvs(uvs)
            .bones(bones)
            .weights(weights)
            .texture(texture)
            .count(count)
            .done();
        self
    }
    pub fn clear(self) -> Self {
        RenderingServer::singleton().canvas_item_clear(self.0);
        self
    }
    pub fn set_canvas_group_mode(
        self,
        mode: CanvasGroupMode,
        clear_margin: f32,
        fit_empty: bool,
        fit_margin: f32,
        blur_mipmaps: bool,
    ) -> Self {
        RenderingServer::singleton()
            .canvas_item_set_canvas_group_mode_ex(self.0, mode)
            .clear_margin(clear_margin)
            .fit_empty(fit_empty)
            .fit_margin(fit_margin)
            .blur_mipmaps(blur_mipmaps)
            .done();
        self
    }
    pub fn set_clip(self, clip: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_clip(self.0, clip);
        self
    }
    pub fn set_copy_to_backbuffer(self, enabled: bool, rect: Rect2) -> Self {
        RenderingServer::singleton().canvas_item_set_copy_to_backbuffer(self.0, enabled, rect);
        self
    }
    pub fn set_custom_rect(self, use_custom_rect: bool, rect: Rect2) -> Self {
        RenderingServer::singleton()
            .canvas_item_set_custom_rect_ex(self.0, use_custom_rect)
            .rect(rect)
            .done();
        self
    }
    pub fn set_default_texture_filter(self, filter: CanvasItemTextureFilter) -> Self {
        RenderingServer::singleton().canvas_item_set_default_texture_filter(self.0, filter);
        self
    }
    pub fn set_default_texture_repeat(self, repeat: CanvasItemTextureRepeat) -> Self {
        RenderingServer::singleton().canvas_item_set_default_texture_repeat(self.0, repeat);
        self
    }
    pub fn set_distance_field_mode(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_distance_field_mode(self.0, enabled);
        self
    }
    pub fn set_draw_behing_parent(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_draw_behind_parent(self.0, enabled);
        self
    }
    pub fn set_draw_index(self, index: i32) -> Self {
        RenderingServer::singleton().canvas_item_set_draw_index(self.0, index);
        self
    }
    pub fn set_light_mask(self, mask: i32) -> Self {
        RenderingServer::singleton().canvas_item_set_light_mask(self.0, mask);
        self
    }
    pub fn set_material(self, material: Rid) -> Self {
        RenderingServer::singleton().canvas_item_set_material(self.0, material);
        self
    }
    pub fn set_modulate(self, color: Color) -> Self {
        RenderingServer::singleton().canvas_item_set_modulate(self.0, color);
        self
    }
    pub fn set_parent(self, parent: Rid) -> Self {
        RenderingServer::singleton().canvas_item_set_parent(self.0, parent);
        self
    }
    pub fn set_self_modulate(self, color: Color) -> Self {
        RenderingServer::singleton().canvas_item_set_self_modulate(self.0, color);
        self
    }
    pub fn set_sort_children_by_y(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_sort_children_by_y(self.0, enabled);
        self
    }
    pub fn set_transform(self, transform: Transform2D) -> Self {
        RenderingServer::singleton().canvas_item_set_transform(self.0, transform);
        self
    }
    pub fn set_use_parent_material(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_use_parent_material(self.0, enabled);
        self
    }
    pub fn set_visibility_layer(self, visibility_layer: u32) -> Self {
        RenderingServer::singleton().canvas_item_set_visibility_layer(self.0, visibility_layer);
        self
    }
    pub fn set_visibility_notifier(
        self,
        enable: bool,
        area: Rect2,
        enter_callable: Callable,
        exit_callable: Callable,
    ) -> Self {
        RenderingServer::singleton().canvas_item_set_visibility_notifier(
            self.0,
            enable,
            area,
            enter_callable,
            exit_callable,
        );
        self
    }
    pub fn set_visible(self, visible: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_visible(self.0, visible);
        self
    }
    pub fn set_z_as_relative_to_parent(self, enabled: bool) -> Self {
        RenderingServer::singleton().canvas_item_set_z_as_relative_to_parent(self.0, enabled);
        self
    }
    pub fn set_z_index(self, z_index: i32) -> Self {
        RenderingServer::singleton().canvas_item_set_z_index(self.0, z_index);
        self
    }
}

impl Default for CanvasItem {
    fn default() -> Self {
        Self::new()
    }
}
