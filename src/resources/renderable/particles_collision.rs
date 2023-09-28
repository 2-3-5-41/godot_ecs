use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    ParticlesCollision,
    particles_collision_create,
    RenderingServer
);

// TODO: Provide a builder API for `ParticlesCollision`
impl ParticlesCollision {}