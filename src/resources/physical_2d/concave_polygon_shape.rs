use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(
    ConcavePolygonShape2D,
    concave_polygon_shape_create,
    PhysicsServer2D
);
