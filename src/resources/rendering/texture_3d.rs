use godot::{
    engine::{image::Format, Image, RenderingServer},
    prelude::{Array, Gd, InstanceId, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone)]
pub struct Texture3D {
    rid: Rid,
    data: Vec<InstanceId>,
    width: i32,
    height: i32,
    depth: i32,
}

impl ResourceId for Texture3D {
    fn get_rid(&self) -> Rid {
        self.rid
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
        let mut array_to_vec: Vec<InstanceId> = vec![];

        data.iter_shared()
            .for_each(|image| array_to_vec.push(image.instance_id()));

        Self {
            rid: RenderingServer::singleton()
                .texture_3d_create(format, width, height, depth, mipmaps, data),
            data: array_to_vec,
            width,
            height,
            depth,
        }
    }
    pub fn from_rid(rid: Rid) -> Self {
        let mut array_to_vec: Vec<InstanceId> = vec![];
        let texture_3d = RenderingServer::singleton().texture_3d_get(rid);

        texture_3d
            .iter_shared()
            .for_each(|image| array_to_vec.push(image.instance_id()));

        Self {
            rid,
            data: array_to_vec,
            width: 0,
            height: 0,
            depth: 0,
        }
    }
    #[deprecated(
        note = "Please use `get_data` instead to access the image data inside the ECS world."
    )]
    pub fn get(&self) -> Array<Gd<Image>> {
        RenderingServer::singleton().texture_3d_get(self.get_rid())
    }
    pub fn get_data(&self) -> Array<Gd<Image>> {
        let mut vec_to_array: Array<Gd<Image>> = Array::default();

        self.data
            .iter()
            .for_each(|instance| vec_to_array.push(Gd::from_instance_id(instance.to_owned())));

        vec_to_array
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }
    pub fn get_depth(&self) -> i32 {
        self.depth
    }
    pub fn update(&self, data: Array<Gd<Image>>) -> &Self {
        RenderingServer::singleton().texture_3d_update(self.get_rid(), data);
        self
    }
}
