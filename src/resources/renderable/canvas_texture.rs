use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(CanvasTexture, canvas_texture_create);

// TODO: Provide a builder API for `CanvasTexture`
impl CanvasTexture {}
