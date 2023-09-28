use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(SegmentShape, segment_shape_create, PhysicsServer2D);
