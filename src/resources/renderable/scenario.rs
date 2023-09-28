use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{
    traits::{RenderableObj, ResourceId},
    utils::macros::renderable_object,
};

renderable_object!(Scenario, scenario_create);

// TODO: Provide a builder API for `Scenario`
impl Scenario {}
