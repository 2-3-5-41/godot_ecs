use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Mesh, mesh_create, RenderingServer);

// TODO: Provide a builder API for `Mesh`
impl Mesh {}
