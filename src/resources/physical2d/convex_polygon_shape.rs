use godot::{engine::PhysicsServer2D, prelude::Rid};
use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(ConvexPolygonShape2D, convex_polygon_shape_create, PhysicsServer2D);
