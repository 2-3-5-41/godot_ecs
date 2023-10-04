use godot::{
    engine::{
        rendering_server::{
            CanvasItemTextureFilter, CanvasItemTextureRepeat, ViewportClearMode, ViewportDebugDraw,
            ViewportEnvironmentMode, ViewportMSAA, ViewportOcclusionCullingBuildQuality,
            ViewportRenderInfo, ViewportRenderInfoType, ViewportSDFOversize, ViewportSDFScale,
            ViewportScaling3DMode, ViewportScreenSpaceAA, ViewportUpdateMode, ViewportVRSMode,
        },
        RenderingServer,
    },
    prelude::{Rect2, Rid, Transform2D},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{camera::Camera, canvas::Canvas, scenario::Scenario};

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
    pub fn attach_to_screen(&self, rect: Rect2, screen: i32) -> &Self {
        RenderingServer::singleton()
            .viewport_attach_to_screen_ex(self.get_rid())
            .rect(rect)
            .screen(screen)
            .done();
        self
    }
    pub fn get_measured_render_time_cpu(&self) -> f64 {
        RenderingServer::singleton().viewport_get_measured_render_time_cpu(self.get_rid())
    }
    pub fn get_measured_render_time_gpu(&self) -> f64 {
        RenderingServer::singleton().viewport_get_measured_render_time_gpu(self.get_rid())
    }
    pub fn get_render_info(
        &self,
        info_type: ViewportRenderInfoType,
        info: ViewportRenderInfo,
    ) -> i32 {
        RenderingServer::singleton().viewport_get_render_info(self.get_rid(), info_type, info)
    }
    pub fn get_render_target(&self) -> Rid {
        RenderingServer::singleton().viewport_get_render_target(self.get_rid())
    }
    pub fn get_texture(&self) -> Rid {
        RenderingServer::singleton().viewport_get_texture(self.get_rid())
    }
    pub fn remove_canvas(&self, canvas: &Canvas) -> &Self {
        RenderingServer::singleton().viewport_remove_canvas(self.get_rid(), canvas.get_rid());
        self
    }
    pub fn set_active(&self, active: bool) -> &Self {
        RenderingServer::singleton().viewport_set_active(self.get_rid(), active);
        self
    }
    pub fn set_canvas_cull_mask(&self, canvas_cull_mask: u32) -> &Self {
        RenderingServer::singleton()
            .viewport_set_canvas_cull_mask(self.get_rid(), canvas_cull_mask);
        self
    }
    pub fn set_canvas_stacking(&self, canvas: &Canvas, layer: i32, sublayer: i32) -> &Self {
        RenderingServer::singleton().viewport_set_canvas_stacking(
            self.get_rid(),
            canvas.get_rid(),
            layer,
            sublayer,
        );
        self
    }
    pub fn set_canvas_transform(&self, canvas: &Canvas, offset: Transform2D) -> &Self {
        RenderingServer::singleton().viewport_set_canvas_transform(
            self.get_rid(),
            canvas.get_rid(),
            offset,
        );
        self
    }
    pub fn set_clear_mode(&self, clear_mode: ViewportClearMode) -> &Self {
        RenderingServer::singleton().viewport_set_clear_mode(self.get_rid(), clear_mode);
        self
    }
    pub fn set_debug_draw(&self, draw: ViewportDebugDraw) -> &Self {
        RenderingServer::singleton().viewport_set_debug_draw(self.get_rid(), draw);
        self
    }
    pub fn set_default_canvas_item_texure_filter(&self, filter: CanvasItemTextureFilter) -> &Self {
        RenderingServer::singleton()
            .viewport_set_default_canvas_item_texture_filter(self.get_rid(), filter);
        self
    }
    pub fn set_defulat_canvas_item_texture_repeat(&self, repeat: CanvasItemTextureRepeat) -> &Self {
        RenderingServer::singleton()
            .viewport_set_default_canvas_item_texture_repeat(self.get_rid(), repeat);
        self
    }
    pub fn set_disable_2d(&self, disable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_disable_2d(self.get_rid(), disable);
        self
    }
    pub fn set_disable_3d(&self, disable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_disable_3d(self.get_rid(), disable);
        self
    }
    pub fn set_environment_mode(&self, mode: ViewportEnvironmentMode) -> &Self {
        RenderingServer::singleton().viewport_set_environment_mode(self.get_rid(), mode);
        self
    }
    pub fn set_fsr_sharpness(&self, sharpness: f32) -> &Self {
        RenderingServer::singleton().viewport_set_fsr_sharpness(self.get_rid(), sharpness);
        self
    }
    pub fn set_global_canvas_transform(&self, transform: Transform2D) -> &Self {
        RenderingServer::singleton()
            .viewport_set_global_canvas_transform(self.get_rid(), transform);
        self
    }
    pub fn set_measure_render_time(&self, enable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_measure_render_time(self.get_rid(), enable);
        self
    }
    pub fn set_msaa_2d(&self, msaa: ViewportMSAA) -> &Self {
        RenderingServer::singleton().viewport_set_msaa_2d(self.get_rid(), msaa);
        self
    }
    pub fn set_msaa_3d(&self, msaa: ViewportMSAA) -> &Self {
        RenderingServer::singleton().viewport_set_msaa_3d(self.get_rid(), msaa);
        self
    }
    pub fn set_occlusion_culling_build_quality(quality: ViewportOcclusionCullingBuildQuality) {
        RenderingServer::singleton().viewport_set_occlusion_culling_build_quality(quality);
    }
    pub fn set_occlussion_rays_per_thread(rays_per_thread: i32) {
        RenderingServer::singleton().viewport_set_occlusion_rays_per_thread(rays_per_thread);
    }
    pub fn set_parent_viewport(&self, parent_viewport: &Viewport) -> &Self {
        RenderingServer::singleton()
            .viewport_set_parent_viewport(self.get_rid(), parent_viewport.get_rid());
        self
    }
    pub fn set_positional_shadow_atlas_quadrant_subdivision(
        &self,
        quadrant: i32,
        subdivision: i32,
    ) -> &Self {
        RenderingServer::singleton().viewport_set_positional_shadow_atlas_quadrant_subdivision(
            self.get_rid(),
            quadrant,
            subdivision,
        );
        self
    }
    pub fn set_positional_shadow_atlas_size(&self, size: i32, use_16_bits: bool) -> &Self {
        RenderingServer::singleton()
            .viewport_set_positional_shadow_atlas_size_ex(self.get_rid(), size)
            .use_16_bits(use_16_bits)
            .done();
        self
    }
    pub fn set_render_direct_to_screen(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().viewport_set_render_direct_to_screen(self.get_rid(), enabled);
        self
    }
    pub fn set_scaling_3d_mode(&self, scaling_3d_mode: ViewportScaling3DMode) -> &Self {
        RenderingServer::singleton().viewport_set_scaling_3d_mode(self.get_rid(), scaling_3d_mode);
        self
    }
    pub fn set_scaling_3d_scale(&self, scale: f32) -> &Self {
        RenderingServer::singleton().viewport_set_scaling_3d_scale(self.get_rid(), scale);
        self
    }
    pub fn set_scenario(&self, scenario: Scenario) -> &Self {
        RenderingServer::singleton().viewport_set_scenario(self.get_rid(), scenario.get_rid());
        self
    }
    pub fn set_screen_space_aa(&self, mode: ViewportScreenSpaceAA) -> &Self {
        RenderingServer::singleton().viewport_set_screen_space_aa(self.get_rid(), mode);
        self
    }
    pub fn set_sdf_oversize_and_scale(
        &self,
        oversize: ViewportSDFOversize,
        scale: ViewportSDFScale,
    ) -> &Self {
        RenderingServer::singleton().viewport_set_sdf_oversize_and_scale(
            self.get_rid(),
            oversize,
            scale,
        );
        self
    }
    pub fn set_size(&self, width: i32, height: i32) -> &Self {
        RenderingServer::singleton().viewport_set_size(self.get_rid(), width, height);
        self
    }
    pub fn set_snap_2d_transforms_to_pixel(&self, enabled: bool) -> &Self {
        RenderingServer::singleton()
            .viewport_set_snap_2d_transforms_to_pixel(self.get_rid(), enabled);
        self
    }
    pub fn set_snap_2d_verticies_to_pixel(&self, enabled: bool) -> &Self {
        RenderingServer::singleton()
            .viewport_set_snap_2d_vertices_to_pixel(self.get_rid(), enabled);
        self
    }
    pub fn set_texture_mipmap_bias(&self, mipmap_bias: f32) -> &Self {
        RenderingServer::singleton().viewport_set_texture_mipmap_bias(self.get_rid(), mipmap_bias);
        self
    }
    pub fn set_transparent_background(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().viewport_set_transparent_background(self.get_rid(), enabled);
        self
    }
    pub fn set_update_mode(&self, update_mode: ViewportUpdateMode) -> &Self {
        RenderingServer::singleton().viewport_set_update_mode(self.get_rid(), update_mode);
        self
    }
    pub fn set_use_debanding(&self, enable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_use_debanding(self.get_rid(), enable);
        self
    }
    pub fn set_use_occlusion_culling(&self, enable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_use_occlusion_culling(self.get_rid(), enable);
        self
    }
    pub fn set_use_taa(&self, enable: bool) -> &Self {
        RenderingServer::singleton().viewport_set_use_taa(self.get_rid(), enable);
        self
    }
    pub fn set_use_xr(&self, use_xr: bool) -> &Self {
        RenderingServer::singleton().viewport_set_use_xr(self.get_rid(), use_xr);
        self
    }
    pub fn set_vrs_mode(&self, mode: ViewportVRSMode) -> &Self {
        RenderingServer::singleton().viewport_set_vrs_mode(self.get_rid(), mode);
        self
    }
    pub fn set_vrs_texture(&self, texture: Rid) -> &Self {
        RenderingServer::singleton().viewport_set_vrs_texture(self.get_rid(), texture);
        self
    }
}
