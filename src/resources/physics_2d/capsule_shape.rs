use crate::resources::{
    traits::{CommonShape2D, ResourceId},
    utils::macros::resource_object,
};
use godot::{engine::PhysicsServer2D, prelude::Rid};

resource_object!(CapsuleShape2D, capsule_shape_create, PhysicsServer2D);

impl CommonShape2D for CapsuleShape2D {}
