use godot::{
    engine::RenderingServer,
    prelude::{Rid, StringName, Variant},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::shader::Shader;

resource_object!(Material, material_create, RenderingServer);

impl Material {
    pub fn get_param(&self, parameter: StringName) -> Variant {
        RenderingServer::singleton().material_get_param(self.get_rid(), parameter)
    }
    pub fn set_next_pass(&self, next_material: Material) -> &Self {
        RenderingServer::singleton()
            .material_set_next_pass(self.get_rid(), next_material.get_rid());
        self
    }
    pub fn set_param(&self, parameter: StringName, value: Variant) -> &Self {
        RenderingServer::singleton().material_set_param(self.get_rid(), parameter, value);
        self
    }
    pub fn set_render_priority(&self, priority: i32) -> &Self {
        RenderingServer::singleton().material_set_render_priority(self.get_rid(), priority);
        self
    }
    pub fn set_shader(&self, shader: Shader) -> &Self {
        RenderingServer::singleton().material_set_shader(self.get_rid(), shader.get_rid());
        self
    }
}
