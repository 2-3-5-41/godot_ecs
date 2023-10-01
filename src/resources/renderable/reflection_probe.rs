use godot::{
    engine::{
        rendering_server::{ReflectionProbeAmbientMode, ReflectionProbeUpdateMode},
        RenderingServer,
    },
    prelude::{Color, Rid, Vector3},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(ReflectionProbe, reflection_probe_create, RenderingServer);

impl ReflectionProbe {
    pub fn set_ambient_color(&self, color: Color) -> &Self {
        RenderingServer::singleton().reflection_probe_set_ambient_color(self.get_rid(), color);
        self
    }
    pub fn set_ambient_engergy(&self, energy: f32) -> &Self {
        RenderingServer::singleton().reflection_probe_set_ambient_energy(self.get_rid(), energy);
        self
    }
    pub fn set_ambient_mode(&self, mode: ReflectionProbeAmbientMode) -> &Self {
        RenderingServer::singleton().reflection_probe_set_ambient_mode(self.get_rid(), mode);
        self
    }
    pub fn set_as_interior(&self, enable: bool) -> &Self {
        RenderingServer::singleton().reflection_probe_set_as_interior(self.get_rid(), enable);
        self
    }
    pub fn set_cull_mask(&self, layers: u32) -> &Self {
        RenderingServer::singleton().reflection_probe_set_cull_mask(self.get_rid(), layers);
        self
    }
    pub fn set_enable_box_projection(&self, enable: bool) -> &Self {
        RenderingServer::singleton()
            .reflection_probe_set_enable_box_projection(self.get_rid(), enable);
        self
    }
    pub fn set_enable_shadows(&self, enable: bool) -> &Self {
        RenderingServer::singleton().reflection_probe_set_enable_shadows(self.get_rid(), enable);
        self
    }
    pub fn set_intensity(&self, intensity: f32) -> &Self {
        RenderingServer::singleton().reflection_probe_set_intensity(self.get_rid(), intensity);
        self
    }
    pub fn set_max_distance(&self, distance: f32) -> &Self {
        RenderingServer::singleton().reflection_probe_set_max_distance(self.get_rid(), distance);
        self
    }
    pub fn set_mesh_lod_threshold(&self, pixels: f32) -> &Self {
        RenderingServer::singleton()
            .reflection_probe_set_mesh_lod_threshold(self.get_rid(), pixels);
        self
    }
    pub fn set_origin_offset(&self, offset: Vector3) -> &Self {
        RenderingServer::singleton().reflection_probe_set_origin_offset(self.get_rid(), offset);
        self
    }
    pub fn set_resolution(&self, resolution: i32) -> &Self {
        RenderingServer::singleton().reflection_probe_set_resolution(self.get_rid(), resolution);
        self
    }
    pub fn set_size(&self, size: Vector3) -> &Self {
        RenderingServer::singleton().reflection_probe_set_size(self.get_rid(), size);
        self
    }
    pub fn set_update_mode(&self, mode: ReflectionProbeUpdateMode) -> &Self {
        RenderingServer::singleton().reflection_probe_set_update_mode(self.get_rid(), mode);
        self
    }
}
