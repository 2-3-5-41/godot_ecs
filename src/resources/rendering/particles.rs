use godot::{
    engine::{
        rendering_server::{ParticlesDrawOrder, ParticlesMode, ParticlesTransformAlign},
        RenderingServer,
    },
    prelude::{Aabb, Array, Color, Rid, Transform3D, Vector3},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{material::Material, mesh::Mesh};

resource_object!(Particles, particles_create, RenderingServer);

impl Particles {
    pub fn emit(
        &self,
        transform: Transform3D,
        velocity: Vector3,
        color: Color,
        custom: Color,
        emit_flags: u32,
    ) -> &Self {
        RenderingServer::singleton().particles_emit(
            self.get_rid(),
            transform,
            velocity,
            color,
            custom,
            emit_flags,
        );
        self
    }
    pub fn get_current_aabb(&self) -> Aabb {
        RenderingServer::singleton().particles_get_current_aabb(self.get_rid())
    }
    pub fn get_emitting(&self) -> bool {
        RenderingServer::singleton().particles_get_emitting(self.get_rid())
    }
    pub fn is_inavtive(&self) -> bool {
        RenderingServer::singleton().particles_is_inactive(self.get_rid())
    }
    pub fn requrest_process(&self) -> &Self {
        RenderingServer::singleton().particles_request_process(self.get_rid());
        self
    }
    pub fn restart(&self) -> &Self {
        RenderingServer::singleton().particles_restart(self.get_rid());
        self
    }
    pub fn set_amount(&self, amount: i32) -> &Self {
        RenderingServer::singleton().particles_set_amount(self.get_rid(), amount);
        self
    }
    pub fn set_collision_base_size(&self, size: f32) -> &Self {
        RenderingServer::singleton().particles_set_collision_base_size(self.get_rid(), size);
        self
    }
    pub fn set_custom_aabb(&self, aabb: Aabb) -> &Self {
        RenderingServer::singleton().particles_set_custom_aabb(self.get_rid(), aabb);
        self
    }
    pub fn set_draw_order(&self, order: ParticlesDrawOrder) -> &Self {
        RenderingServer::singleton().particles_set_draw_order(self.get_rid(), order);
        self
    }
    pub fn set_draw_pass_mesh(&self, pass: i32, mesh: Mesh) -> &Self {
        RenderingServer::singleton().particles_set_draw_pass_mesh(
            self.get_rid(),
            pass,
            mesh.get_rid(),
        );
        self
    }
    pub fn set_draw_passes(&self, count: i32) -> &Self {
        RenderingServer::singleton().particles_set_draw_passes(self.get_rid(), count);
        self
    }
    pub fn set_emission_transform(&self, transform: Transform3D) -> &Self {
        RenderingServer::singleton().particles_set_emission_transform(self.get_rid(), transform);
        self
    }
    pub fn set_emitting(&self, emitting: bool) -> &Self {
        RenderingServer::singleton().particles_set_emitting(self.get_rid(), emitting);
        self
    }
    pub fn set_explosiveness_ratio(&self, ratio: f32) -> &Self {
        RenderingServer::singleton().particles_set_explosiveness_ratio(self.get_rid(), ratio);
        self
    }
    pub fn set_fixed_fps(&self, fps: i32) -> &Self {
        RenderingServer::singleton().particles_set_fixed_fps(self.get_rid(), fps);
        self
    }
    pub fn set_fractional_delta(&self, enable: bool) -> &Self {
        RenderingServer::singleton().particles_set_fractional_delta(self.get_rid(), enable);
        self
    }
    pub fn set_interpolate(&self, enable: bool) -> &Self {
        RenderingServer::singleton().particles_set_interpolate(self.get_rid(), enable);
        self
    }
    pub fn set_lifetime(&self, lifetime: f64) -> &Self {
        RenderingServer::singleton().particles_set_lifetime(self.get_rid(), lifetime);
        self
    }
    pub fn set_mode(&self, mode: ParticlesMode) -> &Self {
        RenderingServer::singleton().particles_set_mode(self.get_rid(), mode);
        self
    }
    pub fn set_one_shot(&self, one_shot: bool) -> &Self {
        RenderingServer::singleton().particles_set_one_shot(self.get_rid(), one_shot);
        self
    }
    pub fn set_pre_process_time(&self, time: f64) -> &Self {
        RenderingServer::singleton().particles_set_pre_process_time(self.get_rid(), time);
        self
    }
    pub fn set_process_material(&self, material: Material) -> &Self {
        RenderingServer::singleton()
            .particles_set_process_material(self.get_rid(), material.get_rid());
        self
    }
    pub fn set_randomness_ratio(&self, ratio: f32) -> &Self {
        RenderingServer::singleton().particles_set_randomness_ratio(self.get_rid(), ratio);
        self
    }
    pub fn set_speed_scale(&self, scale: f64) -> &Self {
        RenderingServer::singleton().particles_set_speed_scale(self.get_rid(), scale);
        self
    }
    pub fn set_subemitter(&self, subemitter_particles: Particles) -> &Self {
        RenderingServer::singleton()
            .particles_set_subemitter(self.get_rid(), subemitter_particles.get_rid());
        self
    }
    pub fn set_trail_bind_poses(&self, bind_poses: Array<Transform3D>) -> &Self {
        RenderingServer::singleton().particles_set_trail_bind_poses(self.get_rid(), bind_poses);
        self
    }
    pub fn set_trails(&self, enable: bool, length_sec: f32) -> &Self {
        RenderingServer::singleton().particles_set_trails(self.get_rid(), enable, length_sec);
        self
    }
    pub fn set_transform_align(&self, align: ParticlesTransformAlign) -> &Self {
        RenderingServer::singleton().particles_set_transform_align(self.get_rid(), align);
        self
    }
    pub fn set_use_local_coordinate(&self, enable: bool) -> &Self {
        RenderingServer::singleton().particles_set_use_local_coordinates(self.get_rid(), enable);
        self
    }
}
