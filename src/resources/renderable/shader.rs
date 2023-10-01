use godot::{
    engine::RenderingServer,
    prelude::{GodotString, Rid, StringName},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Shader, shader_create, RenderingServer);

impl Shader {
    pub fn get_code(&self) -> GodotString {
        RenderingServer::singleton().shader_get_code(self.get_rid())
    }
    pub fn get_default_texture_parameter(&self, name: StringName, index: Option<i32>) -> Rid {
        if let Some(index) = index {
            RenderingServer::singleton()
                .shader_get_default_texture_parameter_ex(self.get_rid(), name)
                .index(index)
                .done()
        } else {
            RenderingServer::singleton().shader_get_default_texture_parameter(self.get_rid(), name)
        }
    }
    pub fn get_parameter_default(&self, name: StringName) -> &Self {
        RenderingServer::singleton().shader_get_parameter_default(self.get_rid(), name);
        self
    }
    pub fn set_code(&self, code: GodotString) -> &Self {
        RenderingServer::singleton().shader_set_code(self.get_rid(), code);
        self
    }
    pub fn set_default_texture_parameter(
        &self,
        name: StringName,
        texture: Rid,
        index: Option<i32>,
    ) -> &Self {
        if let Some(index) = index {
            RenderingServer::singleton()
                .shader_set_default_texture_parameter_ex(self.get_rid(), name, texture)
                .index(index)
                .done();
        } else {
            RenderingServer::singleton().shader_set_default_texture_parameter(
                self.get_rid(),
                name,
                texture,
            );
        }
        self
    }
    pub fn set_path_hint(&self, path: GodotString) -> &Self {
        RenderingServer::singleton().shader_set_path_hint(self.get_rid(), path);
        self
    }
}
