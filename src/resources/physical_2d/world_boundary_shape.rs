use crate::resources::{
    traits::{CommonShape2D, ResourceId},
    utils::macros::resource_object,
};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(
    WorldBoundaryShape2D,
    world_boundary_shape_create,
    PhysicsServer2D
);

impl CommonShape2D for WorldBoundaryShape2D {}
