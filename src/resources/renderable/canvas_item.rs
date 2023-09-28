use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(CanvasItem, canvas_item_create, RenderingServer);

// TODO: Provide a builder API for `CanvasItem`
impl CanvasItem {}
