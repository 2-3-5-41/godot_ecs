use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Material, material_create, RenderingServer);

// TODO: Provide a builder API for `Material`
impl Material {}
