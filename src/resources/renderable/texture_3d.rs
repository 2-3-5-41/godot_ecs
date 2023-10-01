use godot::{
    engine::{image::Format, Image, RenderingServer},
    prelude::{Array, Gd, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub struct Texture3D(Rid);

impl ResourceId for Texture3D {
    fn get_rid(&self) -> Rid {
        self.0
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}
impl Texture3D {
    pub fn create(
        format: Format,
        width: i32,
        height: i32,
        depth: i32,
        mipmaps: bool,
        data: Array<Gd<Image>>,
    ) -> Self {
        Self(
            RenderingServer::singleton()
                .texture_3d_create(format, width, height, depth, mipmaps, data),
        )
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self(rid)
    }
    pub fn get(&self) -> Array<Gd<Image>> {
        RenderingServer::singleton().texture_3d_get(self.get_rid())
    }
    pub fn update(&self, data: Array<Gd<Image>>) -> &Self {
        RenderingServer::singleton().texture_3d_update(self.get_rid(), data);
        self
    }
}
