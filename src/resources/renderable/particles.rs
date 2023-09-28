use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Particles, particles_create);

// TODO: Provide a builder API for `Particles`
impl Particles {}
