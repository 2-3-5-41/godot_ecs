use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(
    SeparationRayShape2D,
    separation_ray_shape_create,
    PhysicsServer2D
);
