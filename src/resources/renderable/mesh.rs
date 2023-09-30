use godot::{
    engine::{
        rendering_server::{ArrayFormat, BlendShapeMode, PrimitiveType},
        RenderingServer,
    },
    prelude::{Aabb, Array, Dictionary, PackedByteArray, Rid, Variant, VariantArray},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::material::Material;

resource_object!(Mesh, mesh_create, RenderingServer);

impl Mesh {
    pub fn add_surface(&self, surface: Dictionary) -> &Self {
        RenderingServer::singleton().mesh_add_surface(self.get_rid(), surface);
        self
    }
    pub fn add_surface_from_arrays(
        &self,
        primitive: PrimitiveType,
        arrays: Array<Variant>,
        blend_shapes: Option<Array<Variant>>,
        lods: Option<Dictionary>,
        compress_format: Option<ArrayFormat>,
    ) -> &Self {
        if let (Some(blend_shapes), Some(lods), Some(compress_format)) =
            (blend_shapes, lods, compress_format)
        {
            RenderingServer::singleton()
                .mesh_add_surface_from_arrays_ex(self.get_rid(), primitive, arrays)
                .blend_shapes(blend_shapes)
                .lods(lods)
                .compress_format(compress_format)
                .done()
        } else {
            RenderingServer::singleton().mesh_add_surface_from_arrays(
                self.get_rid(),
                primitive,
                arrays,
            );
        }
        self
    }
    pub fn clear(&self) -> &Self {
        RenderingServer::singleton().mesh_clear(self.get_rid());
        self
    }
    pub fn create_from_surfaces(
        surfaces: Array<Dictionary>,
        blend_shape_count: Option<i32>,
    ) -> Self {
        if let Some(blend_shape_count) = blend_shape_count {
            Self(
                RenderingServer::singleton()
                    .mesh_create_from_surfaces_ex(surfaces)
                    .blend_shape_count(blend_shape_count)
                    .done(),
            )
        } else {
            Self(RenderingServer::singleton().mesh_create_from_surfaces(surfaces))
        }
    }
    pub fn get_blend_shape_count(&self) -> i32 {
        RenderingServer::singleton().mesh_get_blend_shape_count(self.get_rid())
    }
    pub fn get_blend_shape_mode(&self) -> BlendShapeMode {
        RenderingServer::singleton().mesh_get_blend_shape_mode(self.get_rid())
    }
    pub fn get_custom_aabb(&self) -> Aabb {
        RenderingServer::singleton().mesh_get_custom_aabb(self.get_rid())
    }
    pub fn get_surface(&self, surface: i32) -> Dictionary {
        RenderingServer::singleton().mesh_get_surface(self.get_rid(), surface)
    }
    pub fn get_surface_count(&self) -> i32 {
        RenderingServer::singleton().mesh_get_surface_count(self.get_rid())
    }
    pub fn set_blend_shape_mode(&self, mode: BlendShapeMode) -> &Self {
        RenderingServer::singleton().mesh_set_blend_shape_mode(self.get_rid(), mode);
        self
    }
    pub fn set_custom_aabb(&self, aabb: Aabb) -> &Self {
        RenderingServer::singleton().mesh_set_custom_aabb(self.get_rid(), aabb);
        self
    }
    pub fn set_shadow_mesh(&self, shadow_mesh: Mesh) -> &Self {
        RenderingServer::singleton().mesh_set_shadow_mesh(self.get_rid(), shadow_mesh.get_rid());
        self
    }
    pub fn surface_get_arrays(&self, surface: i32) -> VariantArray {
        RenderingServer::singleton().mesh_surface_get_arrays(self.get_rid(), surface)
    }
    pub fn surface_get_blend_shape_arrays(&self, surface: i32) -> Array<VariantArray> {
        RenderingServer::singleton().mesh_surface_get_blend_shape_arrays(self.get_rid(), surface)
    }
    pub fn surface_get_format_attribute_stride(format: ArrayFormat, vertex_count: i32) -> u32 {
        RenderingServer::singleton().mesh_surface_get_format_attribute_stride(format, vertex_count)
    }
    pub fn surface_get_format_offset(
        format: ArrayFormat,
        vertex_count: i32,
        array_index: i32,
    ) -> u32 {
        RenderingServer::singleton().mesh_surface_get_format_offset(
            format,
            vertex_count,
            array_index,
        )
    }
    pub fn surface_get_format_skin_stride(format: ArrayFormat, vertex_count: i32) -> u32 {
        RenderingServer::singleton().mesh_surface_get_format_skin_stride(format, vertex_count)
    }
    pub fn surface_get_format_vertex_strid(format: ArrayFormat, vertex_count: i32) -> u32 {
        RenderingServer::singleton().mesh_surface_get_format_vertex_stride(format, vertex_count)
    }
    pub fn surface_get_material(&self, surface: i32) -> Material {
        Material::from_rid(
            RenderingServer::singleton().mesh_surface_get_material(self.get_rid(), surface),
        )
    }
    pub fn surface_set_material(&self, surface: i32, material: Material) -> &Self {
        RenderingServer::singleton().mesh_surface_set_material(
            self.get_rid(),
            surface,
            material.get_rid(),
        );
        self
    }
    pub fn surface_update_attribute_region(
        &self,
        surface: i32,
        offset: i32,
        data: PackedByteArray,
    ) -> &Self {
        RenderingServer::singleton().mesh_surface_update_attribute_region(
            self.get_rid(),
            surface,
            offset,
            data,
        );
        self
    }
    pub fn surface_update_skin_region(
        &self,
        surface: i32,
        offset: i32,
        data: PackedByteArray,
    ) -> &Self {
        RenderingServer::singleton().mesh_surface_update_skin_region(
            self.get_rid(),
            surface,
            offset,
            data,
        );
        self
    }
    pub fn surface_update_vertex_region(
        &self,
        surface: i32,
        offset: i32,
        data: PackedByteArray,
    ) -> &Self {
        RenderingServer::singleton().mesh_surface_update_vertex_region(
            self.get_rid(),
            surface,
            offset,
            data,
        );
        self
    }
}
