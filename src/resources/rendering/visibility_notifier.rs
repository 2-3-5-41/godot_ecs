use godot::{
    engine::RenderingServer,
    prelude::{Aabb, Callable, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    VisibilityNotifier,
    visibility_notifier_create,
    RenderingServer
);

impl VisibilityNotifier {
    pub fn set_aabb(&self, aabb: Aabb) -> &Self {
        RenderingServer::singleton().visibility_notifier_set_aabb(self.get_rid(), aabb);
        self
    }
    pub fn set_callbacks(&self, enter_callable: Callable, exit_callable: Callable) -> &Self {
        RenderingServer::singleton().visibility_notifier_set_callbacks(
            self.get_rid(),
            enter_callable,
            exit_callable,
        );
        self
    }
}
