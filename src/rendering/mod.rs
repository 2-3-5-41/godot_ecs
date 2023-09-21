pub mod components;
pub mod resource;

pub trait Renderable {
    fn new() -> Self;
    fn new_from_existing(rid: Option<u64>) -> Self;
    fn get_resource(&self) -> &resource::RendererResource;
}
