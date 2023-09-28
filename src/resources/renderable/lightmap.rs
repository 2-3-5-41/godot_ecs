use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Lightmap, lightmap_create, RenderingServer);

// TODO: Provide a builder API for `Lightmap`
impl Lightmap {}
