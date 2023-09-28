use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    CanvasLightOccluder,
    canvas_light_occluder_create,
    RenderingServer
);

// TODO: Provide a builder API for `CanvasLightOccluder`
impl CanvasLightOccluder {}
