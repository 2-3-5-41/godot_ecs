use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(SeparationRayShape2D, separation_ray_shape_create, PhysicsServer2D);