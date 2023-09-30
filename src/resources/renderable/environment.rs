use godot::{
    engine::{
        rendering_server::{
            EnvironmentAmbientSource, EnvironmentBG, EnvironmentGlowBlendMode,
            EnvironmentReflectionSource, EnvironmentSDFGIFramesToConverge,
            EnvironmentSDFGIFramesToUpdateLight, EnvironmentSDFGIRayCount, EnvironmentSDFGIYScale,
            EnvironmentSSAOQuality, EnvironmentSSILQuality, EnvironmentSSRRoughnessQuality,
            EnvironmentToneMapper,
        },
        RenderingServer,
    },
    prelude::{Basis, Color, PackedFloat32Array, Rid, Vector2i},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::sky::Sky;

resource_object!(Environment, environment_create, RenderingServer);

impl Environment {
    pub fn bake_panorama(&self, bake_irradiance: bool, size: Vector2i) -> &Self {
        RenderingServer::singleton().environment_bake_panorama(
            self.get_rid(),
            bake_irradiance,
            size,
        );
        self
    }
    pub fn set_adjustment(
        &self,
        enable: bool,
        brightness: f32,
        contrast: f32,
        saturation: f32,
        use_1d_color_correction: bool,
        color_correction: Rid,
    ) -> &Self {
        RenderingServer::singleton().environment_set_adjustment(
            self.get_rid(),
            enable,
            brightness,
            contrast,
            saturation,
            use_1d_color_correction,
            color_correction,
        );
        self
    }
    pub fn set_ambient_light(
        &self,
        color: Color,
        ambient: Option<EnvironmentAmbientSource>,
        energy: Option<f32>,
        sky_contribution: Option<f32>,
        reflection_source: Option<EnvironmentReflectionSource>,
    ) -> &Self {
        if let (Some(ambient), Some(energy), Some(sky_contribution), Some(reflection_source)) =
            (ambient, energy, sky_contribution, reflection_source)
        {
            RenderingServer::singleton()
                .environment_set_ambient_light_ex(self.get_rid(), color)
                .ambient(ambient)
                .energy(energy)
                .sky_contibution(sky_contribution)
                .reflection_source(reflection_source)
                .done();
        } else {
            RenderingServer::singleton().environment_set_ambient_light(self.get_rid(), color);
        }
        self
    }
    pub fn set_background(&self, bg: EnvironmentBG) -> &Self {
        RenderingServer::singleton().environment_set_background(self.get_rid(), bg);
        self
    }
    pub fn set_bg_color(&self, color: Color) -> &Self {
        RenderingServer::singleton().environment_set_bg_color(self.get_rid(), color);
        self
    }
    pub fn set_bg_energy(&self, multiplier: f32, exposure_value: f32) -> &Self {
        RenderingServer::singleton().environment_set_bg_energy(
            self.get_rid(),
            multiplier,
            exposure_value,
        );
        self
    }
    pub fn set_canvas_max_layer(&self, max_layer: i32) -> &Self {
        RenderingServer::singleton().environment_set_canvas_max_layer(self.get_rid(), max_layer);
        self
    }
    pub fn set_fog(
        &self,
        enable: bool,
        light_color: Color,
        light_energy: f32,
        sun_scatter: f32,
        density: f32,
        height: f32,
        height_density: f32,
        aerial_perspective: f32,
        sky_affect: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_fog(
            self.get_rid(),
            enable,
            light_color,
            light_energy,
            sun_scatter,
            density,
            height,
            height_density,
            aerial_perspective,
            sky_affect,
        );
        self
    }
    pub fn set_glow(
        &self,
        enable: bool,
        levels: PackedFloat32Array,
        intensity: f32,
        strength: f32,
        mix: f32,
        bloom_threshold: f32,
        blend_mode: EnvironmentGlowBlendMode,
        hdr_bleed_threshold: f32,
        hdr_bleed_scale: f32,
        hdr_luminance_cap: f32,
        glow_map_strength: f32,
        glow_map: Rid,
    ) -> &Self {
        RenderingServer::singleton().environment_set_glow(
            self.get_rid(),
            enable,
            levels,
            intensity,
            strength,
            mix,
            bloom_threshold,
            blend_mode,
            hdr_bleed_threshold,
            hdr_bleed_scale,
            hdr_luminance_cap,
            glow_map_strength,
            glow_map,
        );
        self
    }
    pub fn set_sdfgi(
        &self,
        enable: bool,
        cascades: i32,
        min_cell_size: f32,
        y_scale: EnvironmentSDFGIYScale,
        use_occlusion: bool,
        bounce_feedback: f32,
        read_sky: bool,
        energy: f32,
        normal_bias: f32,
        probe_bias: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_sdfgi(
            self.get_rid(),
            enable,
            cascades,
            min_cell_size,
            y_scale,
            use_occlusion,
            bounce_feedback,
            read_sky,
            energy,
            normal_bias,
            probe_bias,
        );
        self
    }
    pub fn set_sdfgi_frames_to_converge(frames: EnvironmentSDFGIFramesToConverge) {
        RenderingServer::singleton().environment_set_sdfgi_frames_to_converge(frames);
    }
    pub fn set_sdfgi_frames_to_update_light(frames: EnvironmentSDFGIFramesToUpdateLight) {
        RenderingServer::singleton().environment_set_sdfgi_frames_to_update_light(frames);
    }
    pub fn set_sdfgi_ray_count(ray_count: EnvironmentSDFGIRayCount) {
        RenderingServer::singleton().environment_set_sdfgi_ray_count(ray_count);
    }
    pub fn set_sky(&self, sky: Sky) -> &Self {
        RenderingServer::singleton().environment_set_sky(self.get_rid(), sky.get_rid());
        self
    }
    pub fn set_sky_custom_fov(&self, scale: f32) -> &Self {
        RenderingServer::singleton().environment_set_sky_custom_fov(self.get_rid(), scale);
        self
    }
    pub fn set_sky_orientation(&self, orientation: Basis) -> &Self {
        RenderingServer::singleton().environment_set_sky_orientation(self.get_rid(), orientation);
        self
    }
    pub fn set_ssao(
        &self,
        enable: bool,
        radius: f32,
        intensity: f32,
        power: f32,
        detail: f32,
        horizon: f32,
        sharpness: f32,
        light_affect: f32,
        ao_channel_affect: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_ssao(
            self.get_rid(),
            enable,
            radius,
            intensity,
            power,
            detail,
            horizon,
            sharpness,
            light_affect,
            ao_channel_affect,
        );
        self
    }
    pub fn set_ssao_quality(
        quality: EnvironmentSSAOQuality,
        half_size: bool,
        adaptive_target: f32,
        blur_passes: i32,
        fadeout_from: f32,
        fadeout_to: f32,
    ) {
        RenderingServer::singleton().environment_set_ssao_quality(
            quality,
            half_size,
            adaptive_target,
            blur_passes,
            fadeout_from,
            fadeout_to,
        );
    }
    pub fn set_ssil_quality(
        &self,
        quality: EnvironmentSSILQuality,
        half_size: bool,
        adaptive_target: f32,
        blur_passes: i32,
        fadeout_from: f32,
        fadeout_to: f32,
    ) {
        RenderingServer::singleton().environment_set_ssil_quality(
            quality,
            half_size,
            adaptive_target,
            blur_passes,
            fadeout_from,
            fadeout_to,
        );
    }
    pub fn set_ssr(
        &self,
        enable: bool,
        max_steps: i32,
        fade_in: f32,
        fade_out: f32,
        depth_tolerance: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_ssr(
            self.get_rid(),
            enable,
            max_steps,
            fade_in,
            fade_out,
            depth_tolerance,
        );
        self
    }
    pub fn set_ssr_roughness_quality(quality: EnvironmentSSRRoughnessQuality) {
        RenderingServer::singleton().environment_set_ssr_roughness_quality(quality);
    }
    pub fn set_tonemap(
        &self,
        tone_mapper: EnvironmentToneMapper,
        exposure: f32,
        white: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_tonemap(
            self.get_rid(),
            tone_mapper,
            exposure,
            white,
        );
        self
    }
    pub fn set_volumetric_fog(
        &self,
        enable: bool,
        density: f32,
        albedo: Color,
        emission: Color,
        emission_energy: f32,
        anisotropy: f32,
        length: f32,
        p_detail_spread: f32,
        gi_inject: f32,
        temporal_reprojection: bool,
        temporal_reprojection_amount: f32,
        ambient_inject: f32,
        sky_affect: f32,
    ) -> &Self {
        RenderingServer::singleton().environment_set_volumetric_fog(
            self.get_rid(),
            enable,
            density,
            albedo,
            emission,
            emission_energy,
            anisotropy,
            length,
            p_detail_spread,
            gi_inject,
            temporal_reprojection,
            temporal_reprojection_amount,
            ambient_inject,
            sky_affect,
        );
        self
    }
    pub fn set_volumetric_fog_filter_active(active: bool) {
        RenderingServer::singleton().environment_set_volumetric_fog_filter_active(active);
    }
    pub fn set_volumetric_fog_volume_size(size: i32, depth: i32) {
        RenderingServer::singleton().environment_set_volumetric_fog_volume_size(size, depth);
    }
}
