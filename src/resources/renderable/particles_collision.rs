use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(ParticlesCollision, particles_collision_create);

// TODO: Provide a builder API for `ParticlesCollision`
impl ParticlesCollision {}
