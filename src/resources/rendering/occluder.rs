use godot::{
    engine::RenderingServer,
    prelude::{PackedInt32Array, PackedVector3Array, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Occluder, occluder_create, RenderingServer);

impl Occluder {
    pub fn set_mesh(&self, vertices: PackedVector3Array, indices: PackedInt32Array) -> &Self {
        RenderingServer::singleton().occluder_set_mesh(self.get_rid(), vertices, indices);
        self
    }
}
