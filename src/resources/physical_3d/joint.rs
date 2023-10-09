use godot::{
    engine::{
        physics_server_3d::{
            ConeTwistJointParam, G6DOFJointAxisFlag, G6DOFJointAxisParam, HingeJointFlag,
            HingeJointParam, JointType, PinJointParam, SliderJointParam,
        },
        PhysicsServer3D,
    },
    prelude::{Rid, Transform3D, Vector3, Vector3Axis},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::body::Body3D;

resource_object!(Joint3D, joint_create, PhysicsServer3D);

impl Joint3D {
    pub fn cone_twist_joint_get_param(&self, param: ConeTwistJointParam) -> f32 {
        PhysicsServer3D::singleton().cone_twist_joint_get_param(self.get_rid(), param)
    }
    pub fn cone_twist_joint_set_param(&self, param: ConeTwistJointParam, value: f32) -> &Self {
        PhysicsServer3D::singleton().cone_twist_joint_set_param(self.get_rid(), param, value);
        self
    }
    pub fn generic_6dof_joint_get_flag(&self, axis: Vector3Axis, flag: G6DOFJointAxisFlag) -> bool {
        PhysicsServer3D::singleton().generic_6dof_joint_get_flag(self.get_rid(), axis, flag)
    }
    pub fn generic_6dof_joint_get_param(
        &self,
        axis: Vector3Axis,
        param: G6DOFJointAxisParam,
    ) -> f32 {
        PhysicsServer3D::singleton().generic_6dof_joint_get_param(self.get_rid(), axis, param)
    }
    pub fn generic_6dof_joint_set_flag(
        &self,
        axis: Vector3Axis,
        flag: G6DOFJointAxisFlag,
        enable: bool,
    ) -> &Self {
        PhysicsServer3D::singleton().generic_6dof_joint_set_flag(
            self.get_rid(),
            axis,
            flag,
            enable,
        );
        self
    }
    pub fn generic_6dof_joint_set_param(
        &self,
        axis: Vector3Axis,
        param: G6DOFJointAxisParam,
        value: f32,
    ) -> &Self {
        PhysicsServer3D::singleton().generic_6dof_joint_set_param(
            self.get_rid(),
            axis,
            param,
            value,
        );
        self
    }
    pub fn hinge_joint_get_flag(&self, flag: HingeJointFlag) -> bool {
        PhysicsServer3D::singleton().hinge_joint_get_flag(self.get_rid(), flag)
    }
    pub fn hinge_joint_get_param(&self, param: HingeJointParam) -> f32 {
        PhysicsServer3D::singleton().hinge_joint_get_param(self.get_rid(), param)
    }
    pub fn hinge_joint_set_flag(&self, flag: HingeJointFlag, enabled: bool) -> &Self {
        PhysicsServer3D::singleton().hinge_joint_set_flag(self.get_rid(), flag, enabled);
        self
    }
    pub fn hinge_joint_set_param(&self, param: HingeJointParam, value: f32) -> &Self {
        PhysicsServer3D::singleton().hinge_joint_set_param(self.get_rid(), param, value);
        self
    }
    pub fn clear(&self) -> &Self {
        PhysicsServer3D::singleton().joint_clear(self.get_rid());
        self
    }
    pub fn disable_collisions_between_bodies(&self, disable: bool) -> &Self {
        PhysicsServer3D::singleton()
            .joint_disable_collisions_between_bodies(self.get_rid(), disable);
        self
    }
    pub fn get_solver_priority(&self) -> i32 {
        PhysicsServer3D::singleton().joint_get_solver_priority(self.get_rid())
    }
    pub fn get_type(&self) -> JointType {
        PhysicsServer3D::singleton().joint_get_type(self.get_rid())
    }
    pub fn is_disabled_collisions_between_bodies(&self) -> bool {
        PhysicsServer3D::singleton().joint_is_disabled_collisions_between_bodies(self.get_rid())
    }
    pub fn make_cone_twist(
        &self,
        body_a: Body3D,
        local_ref_a: Transform3D,
        body_b: Body3D,
        local_ref_b: Transform3D,
    ) -> &Self {
        PhysicsServer3D::singleton().joint_make_cone_twist(
            self.get_rid(),
            body_a.get_rid(),
            local_ref_a,
            body_b.get_rid(),
            local_ref_b,
        );
        self
    }
    pub fn make_generic_6dof(
        &self,
        body_a: Body3D,
        local_ref_a: Transform3D,
        body_b: Body3D,
        local_ref_b: Transform3D,
    ) -> &Self {
        PhysicsServer3D::singleton().joint_make_generic_6dof(
            self.get_rid(),
            body_a.get_rid(),
            local_ref_a,
            body_b.get_rid(),
            local_ref_b,
        );
        self
    }
    pub fn make_hinge(
        &self,
        body_a: Body3D,
        hinge_a: Transform3D,
        body_b: Body3D,
        hinge_b: Transform3D,
    ) -> &Self {
        PhysicsServer3D::singleton().joint_make_hinge(
            self.get_rid(),
            body_a.get_rid(),
            hinge_a,
            body_b.get_rid(),
            hinge_b,
        );
        self
    }
    pub fn make_pin(
        &self,
        body_a: Body3D,
        local_a: Vector3,
        body_b: Body3D,
        local_b: Vector3,
    ) -> &Self {
        PhysicsServer3D::singleton().joint_make_pin(
            self.get_rid(),
            body_a.get_rid(),
            local_a,
            body_b.get_rid(),
            local_b,
        );
        self
    }
    pub fn make_slider(
        &self,
        body_a: Body3D,
        local_ref_a: Transform3D,
        body_b: Body3D,
        local_ref_b: Transform3D,
    ) -> &Self {
        PhysicsServer3D::singleton().joint_make_slider(
            self.get_rid(),
            body_a.get_rid(),
            local_ref_a,
            body_b.get_rid(),
            local_ref_b,
        );
        self
    }
    pub fn set_solver_priority(&self, priority: i32) -> &Self {
        PhysicsServer3D::singleton().joint_set_solver_priority(self.get_rid(), priority);
        self
    }
    pub fn pin_joint_get_local_a(&self) -> Vector3 {
        PhysicsServer3D::singleton().pin_joint_get_local_a(self.get_rid())
    }
    pub fn pin_joint_get_local_b(&self) -> Vector3 {
        PhysicsServer3D::singleton().pin_joint_get_local_b(self.get_rid())
    }
    pub fn pin_joint_get_param(&self, param: PinJointParam) -> f32 {
        PhysicsServer3D::singleton().pin_joint_get_param(self.get_rid(), param)
    }
    pub fn pin_joint_set_local_a(&self, local_a: Vector3) -> &Self {
        PhysicsServer3D::singleton().pin_joint_set_local_a(self.get_rid(), local_a);
        self
    }
    pub fn pin_joint_set_local_b(&self, local_b: Vector3) -> &Self {
        PhysicsServer3D::singleton().pin_joint_set_local_b(self.get_rid(), local_b);
        self
    }
    pub fn pin_joint_set_param(&self, param: PinJointParam, value: f32) -> &Self {
        PhysicsServer3D::singleton().pin_joint_set_param(self.get_rid(), param, value);
        self
    }
    pub fn slider_joint_get_param(&self, param: SliderJointParam) -> f32 {
        PhysicsServer3D::singleton().slider_joint_get_param(self.get_rid(), param)
    }
    pub fn slider_joint_set_param(&self, param: SliderJointParam, value: f32) -> &Self {
        PhysicsServer3D::singleton().slider_joint_set_param(self.get_rid(), param, value);
        self
    }
}
