use godot::{
    engine::{Image, RenderingServer},
    prelude::{Gd, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub struct Texture2D(Rid);

impl ResourceId for Texture2D {
    fn get_rid(&self) -> Rid {
        self.0
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}
impl Texture2D {
    pub fn create(image: Gd<Image>) -> Self {
        Self(RenderingServer::singleton().texture_2d_create(image))
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn get(&self) -> Option<Gd<Image>> {
        RenderingServer::singleton().texture_2d_get(self.get_rid())
    }
}
