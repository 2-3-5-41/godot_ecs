use bevy_ecs::system::Resource;
use godot::prelude::{godot_error, Array, Dictionary, Gd, Transform3D};

use super::{interface::XrInterface, tracker::XrTracker};

#[derive(Resource, Clone)]
pub struct XrServer;

impl XrServer {
    fn singleton() -> Gd<godot::engine::XrServer> {
        godot::engine::XrServer::singleton()
    }
    pub fn add_interface(interface: Gd<godot::engine::XrInterface>) {
        Self::singleton().add_interface(interface)
    }
    pub fn add_tracker(tracker: Gd<godot::engine::XrPositionalTracker>) {
        Self::singleton().add_tracker(tracker)
    }
    pub fn center_on_hmd(rotation_mode: godot::engine::xr_server::RotationMode, keep_height: bool) {
        Self::singleton().center_on_hmd(rotation_mode, keep_height)
    }
    pub fn find_interface(name: &str) -> Option<XrInterface> {
        let Some(interface) = Self::singleton().find_interface(name.into()) else {
            godot_error!("Failed to find interface: {}", name);
            return None;
        };

        Some(XrInterface::new(interface))
    }
    pub fn get_hmd_transform() -> Transform3D {
        Self::singleton().get_hmd_transform()
    }
    pub fn get_interface(idx: i32) -> Option<XrInterface> {
        let Some(interface) = Self::singleton().get_interface(idx) else {
            godot_error!("Failed to find interface at index: {}", idx);
            return None;
        };

        Some(XrInterface::new(interface))
    }
    pub fn get_interface_count() -> i32 {
        Self::singleton().get_interface_count()
    }
    pub fn get_interfaces() -> Array<Dictionary> {
        Self::singleton().get_interfaces()
    }
    pub fn get_reference_frame() -> Transform3D {
        Self::singleton().get_reference_frame()
    }
    pub fn get_tracker(tracker_name: &str) -> Option<XrTracker> {
        let Some(tracker) = Self::singleton().get_tracker(tracker_name.into()) else {
            godot_error!("Faile dto get tracker: {}", tracker_name);
            return None;
        };

        Some(XrTracker::new(tracker))
    }
    pub fn get_trackers(tracker_types: i32) -> Dictionary {
        Self::singleton().get_trackers(tracker_types)
    }
}
