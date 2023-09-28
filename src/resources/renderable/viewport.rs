use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Viewport, viewport_create);

// TODO: Provide a builder API for `Viewport`
impl Viewport {}
