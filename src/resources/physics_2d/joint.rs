use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{
    engine::{
        physics_server_2d::{DampedSpringParam, JointParam, JointType, PinJointParam},
        PhysicsServer2D,
    },
    prelude::{Rid, Vector2},
};

use super::body::Body2D;

resource_object!(Joint2D, joint_create, PhysicsServer2D);

impl Joint2D {
    pub fn damped_spring_joint_get_param(&self, param: DampedSpringParam) -> f32 {
        PhysicsServer2D::singleton().damped_spring_joint_get_param(self.get_rid(), param)
    }
    pub fn damped_spring_joint_set_param(&self, param: DampedSpringParam, value: f32) -> &Self {
        PhysicsServer2D::singleton().damped_spring_joint_set_param(self.get_rid(), param, value);
        self
    }
    pub fn clear(&self) -> &Self {
        PhysicsServer2D::singleton().joint_clear(self.get_rid());
        self
    }
    pub fn disable_collisions_between_bodies(&self, disable: bool) -> &Self {
        PhysicsServer2D::singleton()
            .joint_disable_collisions_between_bodies(self.get_rid(), disable);
        self
    }
    pub fn get_param(&self, param: JointParam) -> f32 {
        PhysicsServer2D::singleton().joint_get_param(self.get_rid(), param)
    }
    pub fn get_type(&self) -> JointType {
        PhysicsServer2D::singleton().joint_get_type(self.get_rid())
    }
    pub fn is_disabled_collisions_between_bodies(&self) -> bool {
        PhysicsServer2D::singleton().joint_is_disabled_collisions_between_bodies(self.get_rid())
    }
    pub fn make_damped_spring(
        &self,
        anchor_a: Vector2,
        anchor_b: Vector2,
        body_a: Body2D,
        body_b: Body2D,
    ) -> &Self {
        PhysicsServer2D::singleton()
            .joint_make_damped_spring_ex(self.get_rid(), anchor_a, anchor_b, body_a.get_rid())
            .body_b(body_b.get_rid())
            .done();
        self
    }
    pub fn make_groove(
        &self,
        groove1_a: Vector2,
        groove2_a: Vector2,
        anchor_b: Vector2,
        body_a: Body2D,
        body_b: Body2D,
    ) -> &Self {
        PhysicsServer2D::singleton()
            .joint_make_groove_ex(self.get_rid(), groove1_a, groove2_a, anchor_b)
            .body_a(body_a.get_rid())
            .body_b(body_b.get_rid())
            .done();
        self
    }
    pub fn make_pin(&self, anchor: Vector2, body_a: Body2D, body_b: Body2D) -> &Self {
        PhysicsServer2D::singleton()
            .joint_make_pin_ex(self.get_rid(), anchor, body_a.get_rid())
            .body_b(body_b.get_rid())
            .done();
        self
    }
    pub fn set_param(&self, param: JointParam, value: f32) -> &Self {
        PhysicsServer2D::singleton().joint_set_param(self.get_rid(), param, value);
        self
    }
    pub fn pin_joint_get_param(&self, param: PinJointParam) -> f32 {
        PhysicsServer2D::singleton().pin_joint_get_param(self.get_rid(), param)
    }
    pub fn pin_joint_set_param(&self, param: PinJointParam, value: f32) -> &Self {
        PhysicsServer2D::singleton().pin_joint_set_param(self.get_rid(), param, value);
        self
    }
}
