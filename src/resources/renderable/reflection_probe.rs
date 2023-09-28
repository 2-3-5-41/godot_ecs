use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(ReflectionProbe, reflection_probe_create, RenderingServer);

// TODO: Provide a builder API for `ReflectionProbe`
impl ReflectionProbe {}
