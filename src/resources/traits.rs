use godot::{
    engine::{
        physics_server_3d::ShapeType,
        rendering_server::{LightBakeMode, LightParam},
        PhysicsServer3D, RenderingServer,
    },
    prelude::{Color, Rid, Variant},
};

pub trait ResourceId: Clone {
    fn get_rid(&self) -> Rid {
        unimplemented!()
    }
    fn free_rid(&self) {
        unimplemented!()
    }
}

pub trait CommonLight3D: ResourceId {
    fn set_bake_mode(&self, bake_mode: LightBakeMode) -> &Self {
        RenderingServer::singleton().light_set_bake_mode(self.get_rid(), bake_mode);
        self
    }
    fn set_color(&self, color: Color) -> &Self {
        RenderingServer::singleton().light_set_color(self.get_rid(), color);
        self
    }
    fn set_cull_mask(&self, mask: u32) -> &Self {
        RenderingServer::singleton().light_set_cull_mask(self.get_rid(), mask);
        self
    }
    fn set_distance_fade(&self, enabled: bool, begin: f32, shadow: f32, length: f32) -> &Self {
        RenderingServer::singleton().light_set_distance_fade(
            self.get_rid(),
            enabled,
            begin,
            shadow,
            length,
        );
        self
    }
    fn set_max_sdfgi_cascade(&self, cascade: u32) -> &Self {
        RenderingServer::singleton().light_set_max_sdfgi_cascade(self.get_rid(), cascade);
        self
    }
    fn set_negative(&self, enable: bool) -> &Self {
        RenderingServer::singleton().light_set_negative(self.get_rid(), enable);
        self
    }
    fn set_param(&self, param: LightParam, value: f32) -> &Self {
        RenderingServer::singleton().light_set_param(self.get_rid(), param, value);
        self
    }
    fn set_projector(&self, texture: Rid) -> &Self {
        RenderingServer::singleton().light_set_projector(self.get_rid(), texture);
        self
    }
    fn set_shadow(&self, enabled: bool) -> &Self {
        RenderingServer::singleton().light_set_shadow(self.get_rid(), enabled);
        self
    }
}

pub trait CommonShape3D: ResourceId {
    fn get_data(&self) -> Variant {
        PhysicsServer3D::singleton().shape_get_data(self.get_rid())
    }
    fn get_type(&self) -> ShapeType {
        PhysicsServer3D::singleton().shape_get_type(self.get_rid())
    }
    fn set_data(&self, data: Variant) -> &Self {
        PhysicsServer3D::singleton().shape_set_data(self.get_rid(), data);
        self
    }
}
pub trait CommonShape2D: ResourceId {}
