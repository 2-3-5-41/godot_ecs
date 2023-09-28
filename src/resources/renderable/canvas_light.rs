use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(CanvasLight, canvas_light_create, RenderingServer);

// TODO: Provide a builder API for `CanvasLight`
impl CanvasLight {}
