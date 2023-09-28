use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    VisibilityNotifier,
    visibility_notifier_create,
    RenderingServer
);

// TODO: Provide a builder API for `VisibilityNotifier`
impl VisibilityNotifier {}
