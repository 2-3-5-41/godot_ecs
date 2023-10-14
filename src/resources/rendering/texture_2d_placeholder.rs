use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    Texture2DPlaceholder,
    texture_2d_placeholder_create,
    RenderingServer
);

// TODO: Provide a builder API for `Texture2DPlaceholder`
impl Texture2DPlaceholder {}
