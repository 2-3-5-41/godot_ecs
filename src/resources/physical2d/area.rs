use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Area2D, area_create, PhysicsServer2D);