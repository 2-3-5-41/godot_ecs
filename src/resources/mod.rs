use godot::prelude::Rid;

pub mod renderable;
pub mod rid_server;

pub trait ResourceId: Clone + Copy {
    fn get_rid(&self) -> Rid;
}
