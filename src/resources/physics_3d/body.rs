use godot::{
    engine::{
        physics_server_3d::{BodyAxis, BodyMode, BodyParameter, BodyState},
        PhysicsDirectBodyState3D, PhysicsServer3D, PhysicsTestMotionParameters3D,
        PhysicsTestMotionResult3D,
    },
    prelude::{Callable, Gd, Rid, Transform3D, Variant, Vector3},
};

use crate::resources::{
    traits::{CommonShape3D, ResourceId},
    utils::macros::resource_object,
};

use super::space::Space3D;

resource_object!(Body3D, body_create, PhysicsServer3D);

impl Body3D {
    pub fn add_collision_exception(&self, excepted_body: Body3D) -> &Self {
        PhysicsServer3D::singleton()
            .body_add_collision_exception(self.get_rid(), excepted_body.get_rid());
        self
    }
    pub fn add_constant_central_force(&self, force: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_add_constant_central_force(self.get_rid(), force);
        self
    }
    pub fn add_constant_force(&self, force: Vector3, position: Vector3) -> &Self {
        PhysicsServer3D::singleton()
            .body_add_constant_force_ex(self.get_rid(), force)
            .position(position)
            .done();
        self
    }
    pub fn add_constant_torque(&self, torque: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_add_constant_torque(self.get_rid(), torque);
        self
    }
    pub fn add_shape(
        &self,
        shape: impl CommonShape3D,
        transform: Transform3D,
        disabled: bool,
    ) -> &Self {
        PhysicsServer3D::singleton()
            .body_add_shape_ex(self.get_rid(), shape.get_rid())
            .transform(transform)
            .disabled(disabled)
            .done();
        self
    }
    pub fn apply_central_force(&self, force: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_apply_central_force(self.get_rid(), force);
        self
    }
    pub fn apply_central_impulse(&self, impulse: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_apply_central_impulse(self.get_rid(), impulse);
        self
    }
    pub fn apply_force(&self, force: Vector3, position: Vector3) -> &Self {
        PhysicsServer3D::singleton()
            .body_apply_force_ex(self.get_rid(), force)
            .position(position)
            .done();
        self
    }
    pub fn apply_impulse(&self, impulse: Vector3, position: Vector3) -> &Self {
        PhysicsServer3D::singleton()
            .body_apply_impulse_ex(self.get_rid(), impulse)
            .position(position)
            .done();
        self
    }
    pub fn apply_torque(&self, torque: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_apply_torque(self.get_rid(), torque);
        self
    }
    pub fn apply_torque_impulse(&self, torque: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_apply_torque_impulse(self.get_rid(), torque);
        self
    }
    pub fn attach_object_instance_id(&self, id: u64) -> &Self {
        PhysicsServer3D::singleton().body_attach_object_instance_id(self.get_rid(), id);
        self
    }
    pub fn clear_shapes(&self) -> &Self {
        PhysicsServer3D::singleton().body_clear_shapes(self.get_rid());
        self
    }
    pub fn get_collision_layer(&self) -> u32 {
        PhysicsServer3D::singleton().body_get_collision_layer(self.get_rid())
    }
    pub fn get_collision_mask(&self) -> u32 {
        PhysicsServer3D::singleton().body_get_collision_mask(self.get_rid())
    }
    pub fn get_collision_priority(&self) -> f32 {
        PhysicsServer3D::singleton().body_get_collision_priority(self.get_rid())
    }
    pub fn get_constant_force(&self) -> Vector3 {
        PhysicsServer3D::singleton().body_get_constant_force(self.get_rid())
    }
    pub fn get_constant_torque(&self) -> Vector3 {
        PhysicsServer3D::singleton().body_get_constant_torque(self.get_rid())
    }
    pub fn get_direct_state(&self) -> Option<Gd<PhysicsDirectBodyState3D>> {
        PhysicsServer3D::singleton().body_get_direct_state(self.get_rid())
    }
    pub fn get_max_contacts_reported(&self) -> i32 {
        PhysicsServer3D::singleton().body_get_max_contacts_reported(self.get_rid())
    }
    pub fn get_mode(&self) -> BodyMode {
        PhysicsServer3D::singleton().body_get_mode(self.get_rid())
    }
    pub fn get_object_instance_id(&self) -> u64 {
        PhysicsServer3D::singleton().body_get_object_instance_id(self.get_rid())
    }
    pub fn get_param(&self, param: BodyParameter) -> Variant {
        PhysicsServer3D::singleton().body_get_param(self.get_rid(), param)
    }
    pub fn get_shape(&self, shape_idx: i32) -> Rid {
        PhysicsServer3D::singleton().body_get_shape(self.get_rid(), shape_idx)
    }
    pub fn get_shape_count(&self) -> i32 {
        PhysicsServer3D::singleton().body_get_shape_count(self.get_rid())
    }
    pub fn get_shape_transform(&self, shape_idx: i32) -> Transform3D {
        PhysicsServer3D::singleton().body_get_shape_transform(self.get_rid(), shape_idx)
    }
    pub fn get_space(&self) -> Space3D {
        Space3D::from_rid(PhysicsServer3D::singleton().body_get_space(self.get_rid()))
    }
    pub fn get_state(&self, state: BodyState) -> Variant {
        PhysicsServer3D::singleton().body_get_state(self.get_rid(), state)
    }
    pub fn is_axis_locked(&self, axis: BodyAxis) -> bool {
        PhysicsServer3D::singleton().body_is_axis_locked(self.get_rid(), axis)
    }
    pub fn is_continuous_collision_detection_enabled(&self) -> bool {
        PhysicsServer3D::singleton().body_is_continuous_collision_detection_enabled(self.get_rid())
    }
    pub fn is_omitting_cource_integration(&self) -> bool {
        PhysicsServer3D::singleton().body_is_omitting_force_integration(self.get_rid())
    }
    pub fn remove_collision_exception(&self, excepted_body: Body3D) -> &Self {
        PhysicsServer3D::singleton()
            .body_remove_collision_exception(self.get_rid(), excepted_body.get_rid());
        self
    }
    pub fn remove_shape(&self, shape_idx: i32) -> &Self {
        PhysicsServer3D::singleton().body_remove_shape(self.get_rid(), shape_idx);
        self
    }
    pub fn reset_mass_properties(&self) -> &Self {
        PhysicsServer3D::singleton().body_reset_mass_properties(self.get_rid());
        self
    }
    pub fn set_axis_lock(&self, axis: BodyAxis, lock: bool) -> &Self {
        PhysicsServer3D::singleton().body_set_axis_lock(self.get_rid(), axis, lock);
        self
    }
    pub fn set_axis_velocity(&self, axis_velocity: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_set_axis_velocity(self.get_rid(), axis_velocity);
        self
    }
    pub fn set_collision_layer(&self, layer: u32) -> &Self {
        PhysicsServer3D::singleton().body_set_collision_layer(self.get_rid(), layer);
        self
    }
    pub fn set_collision_mask(&self, mask: u32) -> &Self {
        PhysicsServer3D::singleton().body_set_collision_mask(self.get_rid(), mask);
        self
    }
    pub fn set_collision_priority(&self, priority: f32) -> &Self {
        PhysicsServer3D::singleton().body_set_collision_priority(self.get_rid(), priority);
        self
    }
    pub fn set_constant_force(&self, force: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_set_constant_force(self.get_rid(), force);
        self
    }
    pub fn set_constant_torque(&self, torque: Vector3) -> &Self {
        PhysicsServer3D::singleton().body_set_constant_torque(self.get_rid(), torque);
        self
    }
    pub fn set_enable_continuous_collision_detection(&self, enable: bool) -> &Self {
        PhysicsServer3D::singleton()
            .body_set_enable_continuous_collision_detection(self.get_rid(), enable);
        self
    }
    pub fn set_force_integration_callback(&self, callable: Callable, userdata: Variant) -> &Self {
        PhysicsServer3D::singleton()
            .body_set_force_integration_callback_ex(self.get_rid(), callable)
            .userdata(userdata)
            .done();
        self
    }
    pub fn set_max_contacts_reported(&self, amount: i32) -> &Self {
        PhysicsServer3D::singleton().body_set_max_contacts_reported(self.get_rid(), amount);
        self
    }
    pub fn set_mode(&self, mode: BodyMode) -> &Self {
        PhysicsServer3D::singleton().body_set_mode(self.get_rid(), mode);
        self
    }
    pub fn set_omit_force_integration(&self, enable: bool) -> &Self {
        PhysicsServer3D::singleton().body_set_omit_force_integration(self.get_rid(), enable);
        self
    }
    pub fn set_param(&self, param: BodyParameter, value: Variant) -> &Self {
        PhysicsServer3D::singleton().body_set_param(self.get_rid(), param, value);
        self
    }
    pub fn set_ray_pickable(&self, enable: bool) -> &Self {
        PhysicsServer3D::singleton().body_set_ray_pickable(self.get_rid(), enable);
        self
    }
    pub fn set_shape(&self, shape_idx: i32, shape: impl CommonShape3D) -> &Self {
        PhysicsServer3D::singleton().body_set_shape(self.get_rid(), shape_idx, shape.get_rid());
        self
    }
    pub fn set_shape_disabled(&self, shape_idx: i32, disabled: bool) -> &Self {
        PhysicsServer3D::singleton().body_set_shape_disabled(self.get_rid(), shape_idx, disabled);
        self
    }
    pub fn set_shape_transform(&self, shape_idx: i32, transform: Transform3D) -> &Self {
        PhysicsServer3D::singleton().body_set_shape_transform(self.get_rid(), shape_idx, transform);
        self
    }
    pub fn set_space(&self, space: Space3D) -> &Self {
        PhysicsServer3D::singleton().body_set_space(self.get_rid(), space.get_rid());
        self
    }
    pub fn set_state(&self, state: BodyState, value: Variant) -> &Self {
        PhysicsServer3D::singleton().body_set_state(self.get_rid(), state, value);
        self
    }
    pub fn test_motion(
        &self,
        parameters: Gd<PhysicsTestMotionParameters3D>,
        result: Gd<PhysicsTestMotionResult3D>,
    ) -> &Self {
        PhysicsServer3D::singleton()
            .body_test_motion_ex(self.get_rid(), parameters)
            .result(result)
            .done();
        self
    }
}
