use bevy_ecs::system::Resource;
use godot::prelude::{godot_error, Array, Dictionary, Gd, Transform3D};

use super::{interface::XrInterface, tracker::XrTracker};

#[derive(Resource, Clone)]
pub struct XrServer;

impl XrServer {
    pub fn server(&self) -> Gd<godot::engine::XrServer> {
        godot::engine::XrServer::singleton()
    }
    pub fn add_interface(&self, interface: Gd<godot::engine::XrInterface>) {
        self.server().add_interface(interface)
    }
    pub fn add_tracker(&self, tracker: Gd<godot::engine::XrPositionalTracker>) {
        self.server().add_tracker(tracker)
    }
    pub fn center_on_hmd(&self, rotation_mode: godot::engine::xr_server::RotationMode, keep_height: bool) {
        self.server().center_on_hmd(rotation_mode, keep_height)
    }
    pub fn find_interface(&self, name: &str) -> Option<XrInterface> {
        let Some(interface) = self.server().find_interface(name.into()) else {
            godot_error!("Failed to find interface: {}", name);
            return None;
        };

        Some(XrInterface::new(interface))
    }
    pub fn get_hmd_transform(&self) -> Transform3D {
        self.server().get_hmd_transform()
    }
    pub fn get_interface(&self, idx: i32) -> Option<XrInterface> {
        let Some(interface) = self.server().get_interface(idx) else {
            godot_error!("Failed to find interface at index: {}", idx);
            return None;
        };

        Some(XrInterface::new(interface))
    }
    pub fn get_interface_count(&self) -> i32 {
        self.server().get_interface_count()
    }
    pub fn get_interfaces(&self) -> Array<Dictionary> {
        self.server().get_interfaces()
    }
    pub fn get_reference_frame(&self) -> Transform3D {
        self.server().get_reference_frame()
    }
    pub fn get_tracker(&self, tracker_name: &str) -> Option<XrTracker> {
        let Some(tracker) = self.server().get_tracker(tracker_name.into()) else {
            godot_error!("Faile dto get tracker: {}", tracker_name);
            return None;
        };

        Some(XrTracker::new(tracker))
    }
    pub fn get_trackers(&self, tracker_types: i32) -> Dictionary {
        self.server().get_trackers(tracker_types)
    }
}
