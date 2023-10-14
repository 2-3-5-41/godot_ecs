use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{CommonLight3D, ResourceId},
    utils::macros::resource_object,
};

resource_object!(SpotLight, spot_light_create, RenderingServer);

impl CommonLight3D for SpotLight {}
