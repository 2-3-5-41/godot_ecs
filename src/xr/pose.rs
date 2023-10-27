use bevy_ecs::component::Component;
use godot::prelude::{Gd, Transform3D};

use crate::gd::GdInstance;

#[derive(Component, Debug, Clone, Copy)]
pub struct XrPose(GdInstance);

impl XrPose {
    pub fn pose(&self) -> Gd<godot::engine::XrPose> {
        self.0.bind::<godot::engine::XrPose>()
    }
    pub fn new(pose: Gd<godot::engine::XrPose>) -> Self {
        Self(GdInstance::new(pose))
    }
    pub fn get_adjusted_transform(&self) -> Transform3D {
        self.pose().get_adjusted_transform()
    }
}
