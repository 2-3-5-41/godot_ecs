use godot::{
    engine::{rendering_server::TextureLayeredType, Image, RenderingServer},
    prelude::{Array, Gd, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub struct Texture2DLayered(Rid);

impl ResourceId for Texture2DLayered {
    fn get_rid(&self) -> Rid {
        self.0
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}
impl Texture2DLayered {
    pub fn create(image: Array<Gd<Image>>, layered_type: TextureLayeredType) -> Self {
        Self(RenderingServer::singleton().texture_2d_layered_create(image, layered_type))
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn layer_get(&self, layer: i32) -> Option<Gd<Image>> {
        RenderingServer::singleton().texture_2d_layer_get(self.get_rid(), layer)
    }
    pub fn update(&self, image: Gd<Image>, layer: i32) -> &Self {
        RenderingServer::singleton().texture_2d_update(self.get_rid(), image, layer);
        self
    }
}
