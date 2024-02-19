use godot::{
    engine::{Image, RenderingServer},
    prelude::{Gd, InstanceId, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub struct Texture2D {
    rid: Rid,
    image: InstanceId,
}

impl ResourceId for Texture2D {
    fn get_rid(&self) -> Rid {
        self.rid
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}
impl Texture2D {
    pub fn get_image(&self) -> Gd<Image> {
        self.try_get_image()
            .expect("Failed to get image from instance id!")
    }
    pub fn try_get_image(&self) -> Option<Gd<Image>> {
        let instance = self.image;
        match Gd::try_from_instance_id(instance) {
            Ok(image) => Some(image),
            // API changed upstream. For now ignore the error.
            Err(_) => None,
        }
    }
    pub fn create(image: Gd<Image>) -> Self {
        let id = image.instance_id();
        Self {
            rid: RenderingServer::singleton().texture_2d_create(image),
            image: id,
        }
    }
    pub fn from_rid(rid: Rid) -> Self {
        Self {
            rid,
            image: RenderingServer::singleton()
                .texture_2d_get(rid)
                .expect("Failed to get Texture2D from RID!")
                .instance_id(),
        }
    }
    #[deprecated(
        note = "Please use `try_get_image` or `get_image` to access the Image in the ECS world."
    )]
    pub fn get(&self) -> Option<Gd<Image>> {
        RenderingServer::singleton().texture_2d_get(self.get_rid())
    }
}
