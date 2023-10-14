use godot::{engine::PhysicsServer3D, prelude::Rid};

use crate::resources::{
    traits::{CommonShape3D, ResourceId},
    utils::macros::resource_object,
};

resource_object!(BoxShape, box_shape_create, PhysicsServer3D);

impl CommonShape3D for BoxShape {}
