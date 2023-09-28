use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(DirectionalLight, directional_light_create, RenderingServer);

// TODO: Provide a builder API for `DirectionalLight`
impl DirectionalLight {}
