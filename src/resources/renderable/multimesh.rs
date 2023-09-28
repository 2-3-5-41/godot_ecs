use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Multimesh, multimesh_create);

// TODO: Provide a builder API for `Multimesh`
impl Multimesh {}
