use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Texture2DPlaceholder, texture_2d_placeholder_create);

// TODO: Provide a builder API for `Texture2DPlaceholder`
impl Texture2DPlaceholder {}
