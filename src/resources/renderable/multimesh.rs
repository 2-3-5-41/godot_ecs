use godot::{
    engine::{rendering_server::MultimeshTransformFormat, RenderingServer},
    prelude::{Aabb, Color, PackedFloat32Array, Rid, Transform2D, Transform3D},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::mesh::Mesh;

resource_object!(Multimesh, multimesh_create, RenderingServer);

impl Multimesh {
    pub fn allocate_data(
        &self,
        instances: i32,
        transform_format: MultimeshTransformFormat,
        color_format: bool,
        custom_data_format: bool,
    ) -> &Self {
        RenderingServer::singleton()
            .multimesh_allocate_data_ex(self.get_rid(), instances, transform_format)
            .color_format(color_format)
            .custom_data_format(custom_data_format)
            .done();
        self
    }
    pub fn get_aabb(&self) -> Aabb {
        RenderingServer::singleton().multimesh_get_aabb(self.get_rid())
    }
    pub fn get_buffer(&self) -> PackedFloat32Array {
        RenderingServer::singleton().multimesh_get_buffer(self.get_rid())
    }
    pub fn get_instance_count(&self) -> i32 {
        RenderingServer::singleton().multimesh_get_instance_count(self.get_rid())
    }
    pub fn get_mesh(&self) -> Rid {
        RenderingServer::singleton().multimesh_get_mesh(self.get_rid())
    }
    pub fn get_visible_instances(&self) -> i32 {
        RenderingServer::singleton().multimesh_get_visible_instances(self.get_rid())
    }
    pub fn instance_get_color(&self, index: i32) -> Color {
        RenderingServer::singleton().multimesh_instance_get_color(self.get_rid(), index)
    }
    pub fn instance_get_custom_data(&self, index: i32) -> Color {
        RenderingServer::singleton().multimesh_instance_get_custom_data(self.get_rid(), index)
    }
    pub fn instance_get_transform(&self, index: i32) -> Transform3D {
        RenderingServer::singleton().multimesh_instance_get_transform(self.get_rid(), index)
    }
    pub fn instance_get_transform_2d(&self, index: i32) -> Transform2D {
        RenderingServer::singleton().multimesh_instance_get_transform_2d(self.get_rid(), index)
    }
    pub fn instance_set_color(&self, index: i32, color: Color) -> &Self {
        RenderingServer::singleton().multimesh_instance_set_color(self.get_rid(), index, color);
        self
    }
    pub fn instance_set_custom_data(&self, index: i32, custom_data: Color) -> &Self {
        RenderingServer::singleton().multimesh_instance_set_custom_data(
            self.get_rid(),
            index,
            custom_data,
        );
        self
    }
    pub fn instance_set_transform(&self, index: i32, transform: Transform3D) -> &Self {
        RenderingServer::singleton().multimesh_instance_set_transform(
            self.get_rid(),
            index,
            transform,
        );
        self
    }
    pub fn instance_set_transform_2d(&self, index: i32, transform: Transform2D) -> &Self {
        RenderingServer::singleton().multimesh_instance_set_transform_2d(
            self.get_rid(),
            index,
            transform,
        );
        self
    }
    pub fn set_buffer(&self, buffer: PackedFloat32Array) -> &Self {
        RenderingServer::singleton().multimesh_set_buffer(self.get_rid(), buffer);
        self
    }
    pub fn set_mesh(&self, mesh: Mesh) -> &Self {
        RenderingServer::singleton().multimesh_set_mesh(self.get_rid(), mesh.get_rid());
        self
    }
    pub fn set_visible_instances(&self, visible: i32) -> &Self {
        RenderingServer::singleton().multimesh_set_visible_instances(self.get_rid(), visible);
        self
    }
}
