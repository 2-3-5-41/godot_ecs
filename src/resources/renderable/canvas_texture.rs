use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(CanvasTexture, canvas_texture_create, RenderingServer);

// TODO: Provide a builder API for `CanvasTexture`
impl CanvasTexture {}
