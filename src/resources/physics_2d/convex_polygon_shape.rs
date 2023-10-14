use crate::resources::{
    traits::{CommonShape2D, ResourceId},
    utils::macros::resource_object,
};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(
    ConvexPolygonShape2D,
    convex_polygon_shape_create,
    PhysicsServer2D
);

impl CommonShape2D for ConvexPolygonShape2D {}
