use bevy_ecs::component::Component;
use godot::{
    engine::xr_pose::TrackingConfidence,
    prelude::{godot_error, Gd, Transform3D, Variant, Vector3},
};

use crate::gd::GdInstance;

use super::pose::XrPose;

#[derive(Component, Debug, Clone, Copy)]
pub struct XrTracker(GdInstance);

impl XrTracker {
    pub fn tracker(&self) -> Gd<godot::engine::XrPositionalTracker> {
        self.0.bind::<godot::engine::XrPositionalTracker>()
    }
    pub fn new(tracker: Gd<godot::engine::XrPositionalTracker>) -> Self {
        Self(GdInstance::new(tracker))
    }
    pub fn get_input(&self, name: &str) -> Variant {
        self.tracker().get_input(name.into())
    }
    pub fn get_pose(&self, name: &str) -> Option<XrPose> {
        let Some(pose) = self.tracker().get_pose(name.into()) else {
            godot_error!("Failed to get pose: {}", name);
            return None;
        };

        Some(XrPose::new(pose))
    }
    pub fn has_pose(&self, name: &str) -> bool {
        self.tracker().has_pose(name.into())
    }
    pub fn invalidate_pose(&self, name: &str) {
        self.tracker().invalidate_pose(name.into());
    }
    pub fn set_input(&self, name: &str, value: Variant) {
        self.tracker().set_input(name.into(), value);
    }
    pub fn set_pose(
        &self,
        name: &str,
        transform: Transform3D,
        linear_velocity: Vector3,
        angular_velocity: Vector3,
        tracking_confidence: TrackingConfidence,
    ) {
        self.tracker().set_pose(
            name.into(),
            transform,
            linear_velocity,
            angular_velocity,
            tracking_confidence,
        );
    }
}
