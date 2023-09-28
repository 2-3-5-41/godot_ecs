use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Multimesh, multimesh_create, RenderingServer);

// TODO: Provide a builder API for `Multimesh`
impl Multimesh {}
