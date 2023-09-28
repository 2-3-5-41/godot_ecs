use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(VisibilityNotifier, visibility_notifier_create);

// TODO: Provide a builder API for `VisibilityNotifier`
impl VisibilityNotifier {}
