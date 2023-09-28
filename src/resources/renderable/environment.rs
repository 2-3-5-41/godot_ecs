use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Environment, environment_create, RenderingServer);

// TODO: Provide a builder API for `Environment`
impl Environment {}
