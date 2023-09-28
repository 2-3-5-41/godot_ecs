use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    Texture3DPlaceholder,
    texture_3d_placeholder_create,
    RenderingServer
);

// TODO: Provide a builder API for `Texture2DPlaceholder`
impl Texture3DPlaceholder {}
