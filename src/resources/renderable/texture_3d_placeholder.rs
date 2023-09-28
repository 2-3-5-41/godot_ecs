use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Texture3DPlaceholder, texture_3d_placeholder_create);

// TODO: Provide a builder API for `Texture2DPlaceholder`
impl Texture3DPlaceholder {}
