use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(CapsuleShape2D, capsule_shape_create, PhysicsServer2D);
