use godot::{engine::RenderingServer, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(CameraAttributes, camera_attributes_create, RenderingServer);

// TODO: Provide a builder API for `CameraAttributes`
impl CameraAttributes {}
