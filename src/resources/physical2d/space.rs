use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Space2D, space_create, PhysicsServer2D);
