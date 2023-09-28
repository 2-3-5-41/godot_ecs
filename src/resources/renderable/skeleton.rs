use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Skeleton, skeleton_create, RenderingServer);

// TODO: Provide a builder API for `Skeleton`
impl Skeleton {}
