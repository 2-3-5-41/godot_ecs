use godot::{engine::PhysicsServer3D, prelude::Rid};

use crate::resources::{traits::{ResourceId, CommonShape3D}, utils::macros::resource_object};

resource_object!(
    ConvexPolygonShape3D,
    convex_polygon_shape_create,
    PhysicsServer3D
);

impl CommonShape3D for ConvexPolygonShape3D {}