use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Body2D, body_create, PhysicsServer2D);
