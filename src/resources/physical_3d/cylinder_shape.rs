use godot::{engine::PhysicsServer3D, prelude::Rid};

use crate::resources::{traits::{ResourceId, CommonShape3D}, utils::macros::resource_object};

resource_object!(CylinderShape, cylinder_shape_create, PhysicsServer3D);

impl CommonShape3D for CylinderShape {}