use godot::{
    engine::{rendering_server::TextureLayeredType, Image, RenderingServer},
    prelude::{Array, Gd, InstanceId, Rid},
};

use crate::resources::traits::ResourceId;

#[derive(Debug, Clone)]
pub struct Texture2DLayered {
    rid: Rid,
    layers: Vec<InstanceId>,
}

impl ResourceId for Texture2DLayered {
    fn get_rid(&self) -> Rid {
        self.rid
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}
impl Texture2DLayered {
    pub fn create(layers: Array<Gd<Image>>, layered_type: TextureLayeredType) -> Self {
        let mut array_to_vec: Vec<InstanceId> = vec![];

        layers
            .iter_shared()
            .for_each(|image| array_to_vec.push(image.instance_id()));

        Self {
            rid: RenderingServer::singleton().texture_2d_layered_create(layers, layered_type),
            layers: array_to_vec,
        }
    }
    pub fn from_rid(rid: Rid, len: i32) -> Self {
        let mut array_to_vec: Vec<InstanceId> = vec![];
        let mut i: i32 = 0;

        while i < len {
            let layer = RenderingServer::singleton()
                .texture_2d_layer_get(rid, i)
                .expect("Failed to get Texture2D layer from RID or layer index!")
                .instance_id();

            array_to_vec.push(layer);

            i = i + 1;
        }

        Self {
            rid,
            layers: array_to_vec,
        }
    }
    #[deprecated(
        note = "Please use `try_get_layer` or `get_layer` to access texture layers in the ECS world."
    )]
    pub fn layer_get(&self, layer: i32) -> Option<Gd<Image>> {
        RenderingServer::singleton().texture_2d_layer_get(self.get_rid(), layer)
    }
    pub fn get_layer(&self, layer: usize) -> Gd<Image> {
        self.try_get_layer(layer).expect("Failed to get layer!")
    }
    pub fn try_get_layer(&self, layer: usize) -> Option<Gd<Image>> {
        let instance = self.layers[layer];
        match Gd::try_from_instance_id(instance) {
            Ok(image) => Some(image),
            // API changed upstream. For now ignore the error.
            Err(_) => None,
        }
    }
    pub fn update(&self, image: Gd<Image>, layer: i32) -> &Self {
        RenderingServer::singleton().texture_2d_update(self.get_rid(), image, layer);
        self
    }
}
