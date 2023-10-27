use bevy_ecs::component::Component;
use godot::{
    engine::xr_interface::{EnvironmentBlendMode, PlayAreaMode, TrackingStatus},
    prelude::{
        Array, Dictionary, Gd, PackedVector3Array, Projection, Transform3D, Variant, Vector2,
    },
};

use crate::gd::GdInstance;

#[derive(Component, Debug, Clone, Copy)]
pub struct XrInterface(GdInstance);

impl XrInterface {
    pub fn interface(&self) -> Gd<godot::engine::XrInterface> {
        self.0.bind::<godot::engine::XrInterface>()
    }
    pub fn new(interface: Gd<godot::engine::XrInterface>) -> Self {
        Self(GdInstance::new(interface))
    }
    pub fn get_camera_feed_id(&self) -> i32 {
        self.interface().get_camera_feed_id()
    }
    pub fn get_capabilities(&self) -> u32 {
        self.interface().get_capabilities()
    }
    pub fn get_name(&self) -> String {
        self.interface().get_name().into()
    }
    pub fn get_play_area(&self) -> PackedVector3Array {
        self.interface().get_play_area()
    }
    pub fn get_projection_for_view(
        &self,
        view: u32,
        aspect: f64,
        near: f64,
        far: f64,
    ) -> Projection {
        self.interface()
            .get_projection_for_view(view, aspect, near, far)
    }
    pub fn get_render_target_size(&self) -> Vector2 {
        self.interface().get_render_target_size()
    }
    pub fn get_supported_environment_blend_modes(&self) -> Array<Variant> {
        self.interface().get_supported_environment_blend_modes()
    }
    pub fn get_system_info(&self) -> Dictionary {
        self.interface().get_system_info()
    }
    pub fn get_tracking_status(&self) -> TrackingStatus {
        self.interface().get_tracking_status()
    }
    pub fn get_transform_for_view(&self, view: u32, cam_transform: Transform3D) -> Transform3D {
        self.interface().get_transform_for_view(view, cam_transform)
    }
    pub fn get_view_count(&self) -> u32 {
        self.interface().get_view_count()
    }
    pub fn initialize(self) -> bool {
        self.interface().initialize()
    }
    pub fn is_passthrough_enabled(&self) -> bool {
        self.interface().is_passthrough_enabled()
    }
    pub fn is_passthrough_supported(&self) -> bool {
        self.interface().is_passthrough_supported()
    }
    pub fn set_environment_blend_mode(&self, mode: EnvironmentBlendMode) -> bool {
        self.interface().set_environment_blend_mode(mode)
    }
    pub fn set_play_area_mode(&self, mode: PlayAreaMode) -> bool {
        self.interface().set_play_area_mode(mode)
    }
    pub fn start_passthrough(&self) -> bool {
        self.interface().start_passthrough()
    }
    pub fn stop_passthrough(&self) {
        self.interface().stop_passthrough()
    }
    pub fn supports_play_area_mode(&self, mode: PlayAreaMode) -> bool {
        self.interface().supports_play_area_mode(mode)
    }
    pub fn trigger_haptic_pulse(
        &self,
        action_name: &str,
        tracker_name: &str,
        frequency: f64,
        amplitude: f64,
        duration_sec: f64,
        delay_sec: f64,
    ) {
        self.interface().trigger_haptic_pulse(
            action_name.into(),
            tracker_name.into(),
            frequency,
            amplitude,
            duration_sec,
            delay_sec,
        )
    }
    pub fn uninitialize(self) {
        self.interface().uninitialize()
    }
}
