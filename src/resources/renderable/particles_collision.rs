use godot::{
    engine::{
        rendering_server::{ParticlesCollisionHeightfieldResolution, ParticlesCollisionType},
        RenderingServer,
    },
    prelude::{Rid, Vector3},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(
    ParticlesCollision,
    particles_collision_create,
    RenderingServer
);

impl ParticlesCollision {
    pub fn height_field_update(&self) -> &Self {
        RenderingServer::singleton().particles_collision_height_field_update(self.get_rid());
        self
    }
    pub fn set_attractor_attenuation(&self, curve: f32) -> &Self {
        RenderingServer::singleton()
            .particles_collision_set_attractor_attenuation(self.get_rid(), curve);
        self
    }
    pub fn set_attractor_directionality(&self, amount: f32) -> &Self {
        RenderingServer::singleton()
            .particles_collision_set_attractor_directionality(self.get_rid(), amount);
        self
    }
    pub fn set_attractor_strength(&self, strength: f32) -> &Self {
        RenderingServer::singleton()
            .particles_collision_set_attractor_strength(self.get_rid(), strength);
        self
    }
    pub fn set_box_extents(&self, extents: Vector3) -> &Self {
        RenderingServer::singleton().particles_collision_set_box_extents(self.get_rid(), extents);
        self
    }
    pub fn set_collision_type(&self, collision_type: ParticlesCollisionType) -> &Self {
        RenderingServer::singleton()
            .particles_collision_set_collision_type(self.get_rid(), collision_type);
        self
    }
    pub fn set_cull_mask(&self, mask: u32) -> &Self {
        RenderingServer::singleton().particles_collision_set_cull_mask(self.get_rid(), mask);
        self
    }
    pub fn set_field_texture(&self, texture: Rid) -> &Self {
        RenderingServer::singleton().particles_collision_set_field_texture(self.get_rid(), texture);
        self
    }
    pub fn set_height_field_resolution(
        &self,
        resolution: ParticlesCollisionHeightfieldResolution,
    ) -> &Self {
        RenderingServer::singleton()
            .particles_collision_set_height_field_resolution(self.get_rid(), resolution);
        self
    }
    pub fn set_sphere_radius(&self, radius: f32) -> &Self {
        RenderingServer::singleton().particles_collision_set_sphere_radius(self.get_rid(), radius);
        self
    }
}
