use godot::{
    engine::{
        rendering_server::{InstanceFlags, ShadowCastingSetting, VisibilityRangeFadeMode},
        RenderingServer,
    },
    prelude::{Aabb, Array, Dictionary, Rect2, Rid, StringName, Transform3D, Variant},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{lightmap::Lightmap, material::Material, scenario::Scenario, skeleton::Skeleton};

resource_object!(Instance, instance_create, RenderingServer);

impl Instance {
    pub fn attach_object_instance_id(&self, id: u64) -> &Self {
        RenderingServer::singleton().instance_attach_object_instance_id(self.get_rid(), id);
        self
    }
    pub fn attach_skeleton(&self, skeleton: Skeleton) -> &Self {
        RenderingServer::singleton().instance_attach_skeleton(self.get_rid(), skeleton.get_rid());
        self
    }
    pub fn geometry_get_shader_parameter(&self, parameter: StringName) -> Variant {
        RenderingServer::singleton()
            .instance_geometry_get_shader_parameter(self.get_rid(), parameter)
    }
    pub fn geometry_get_shader_parameter_default_value(&self, parameter: StringName) -> Variant {
        RenderingServer::singleton()
            .instance_geometry_get_shader_parameter_default_value(self.get_rid(), parameter)
    }
    pub fn geometry_get_shader_parameter_list(&self) -> Array<Dictionary> {
        RenderingServer::singleton().instance_geometry_get_shader_parameter_list(self.get_rid())
    }
    pub fn geometry_set_cast_shadows_setting(
        &self,
        shadow_casting_setting: ShadowCastingSetting,
    ) -> &Self {
        RenderingServer::singleton()
            .instance_geometry_set_cast_shadows_setting(self.get_rid(), shadow_casting_setting);
        self
    }
    pub fn geometry_set_flag(&self, flag: InstanceFlags, enabled: bool) -> &Self {
        RenderingServer::singleton().instance_geometry_set_flag(self.get_rid(), flag, enabled);
        self
    }
    pub fn geometry_set_lightmap(
        &self,
        lightmap: Lightmap,
        lightmap_uv_scale: Rect2,
        lightmap_slice: i32,
    ) -> &Self {
        RenderingServer::singleton().instance_geometry_set_lightmap(
            self.get_rid(),
            lightmap.get_rid(),
            lightmap_uv_scale,
            lightmap_slice,
        );
        self
    }
    pub fn geometry_set_lod_bias(&self, lod_bias: f32) -> &Self {
        RenderingServer::singleton().instance_geometry_set_lod_bias(self.get_rid(), lod_bias);
        self
    }
    pub fn geometry_set_material_overlay(&self, material: Material) -> &Self {
        RenderingServer::singleton()
            .instance_geometry_set_material_overlay(self.get_rid(), material.get_rid());
        self
    }
    pub fn geometry_set_material_override(&self, material: Material) -> &Self {
        RenderingServer::singleton()
            .instance_geometry_set_material_override(self.get_rid(), material.get_rid());
        self
    }
    pub fn geometry_set_shader_parameter(&self, parameter: StringName, value: Variant) -> &Self {
        RenderingServer::singleton().instance_geometry_set_shader_parameter(
            self.get_rid(),
            parameter,
            value,
        );
        self
    }
    pub fn geometry_set_transparency(&self, transparency: f32) -> &Self {
        RenderingServer::singleton()
            .instance_geometry_set_transparency(self.get_rid(), transparency);
        self
    }
    pub fn geometry_set_visibility_range(
        &self,
        min: f32,
        max: f32,
        min_margin: f32,
        max_margin: f32,
        fade_mode: VisibilityRangeFadeMode,
    ) -> &Self {
        RenderingServer::singleton().instance_geometry_set_visibility_range(
            self.get_rid(),
            min,
            max,
            min_margin,
            max_margin,
            fade_mode,
        );
        self
    }
    pub fn set_base(&self, base: Rid) -> &Self {
        RenderingServer::singleton().instance_set_base(self.get_rid(), base);
        self
    }
    pub fn set_blend_shape_weight(&self, shape: i32, weight: f32) -> &Self {
        RenderingServer::singleton().instance_set_blend_shape_weight(self.get_rid(), shape, weight);
        self
    }
    pub fn set_custom_aabb(&self, aabb: Aabb) -> &Self {
        RenderingServer::singleton().instance_set_custom_aabb(self.get_rid(), aabb);
        self
    }
    pub fn set_extra_visibility_margin(&self, margin: f32) -> &Self {
        RenderingServer::singleton().instance_set_extra_visibility_margin(self.get_rid(), margin);
        self
    }
    pub fn set_ignore_culling(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().instance_set_ignore_culling(self.get_rid(), enabled);
        self
    }
    pub fn set_layer_mask(&self, mask: u32) -> &Self {
        RenderingServer::singleton().instance_set_layer_mask(self.get_rid(), mask);
        self
    }
    pub fn set_pivot_data(&self, sorting_offset: f32, use_aabb_center: bool) -> &Self {
        RenderingServer::singleton().instance_set_pivot_data(
            self.get_rid(),
            sorting_offset,
            use_aabb_center,
        );
        self
    }
    pub fn set_scenario(&self, scenario: Scenario) -> &Self {
        RenderingServer::singleton().instance_set_scenario(self.get_rid(), scenario.get_rid());
        self
    }
    pub fn set_surface_override_material(&self, surface: i32, material: Material) -> &Self {
        RenderingServer::singleton().instance_set_surface_override_material(
            self.get_rid(),
            surface,
            material.get_rid(),
        );
        self
    }
    pub fn set_transform(&self, transform: Transform3D) -> &Self {
        RenderingServer::singleton().instance_set_transform(self.get_rid(), transform);
        self
    }
    pub fn set_visibility_parent(&self, parent: Instance) -> &Self {
        RenderingServer::singleton()
            .instance_set_visibility_parent(self.get_rid(), parent.get_rid());
        self
    }
    pub fn set_visible(&self, visible: bool) -> &Self {
        RenderingServer::singleton().instance_set_visible(self.get_rid(), visible);
        self
    }
}
