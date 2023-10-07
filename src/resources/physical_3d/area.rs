use godot::{
    engine::{physics_server_3d::AreaParameter, PhysicsServer3D},
    prelude::{Callable, Rid, Transform3D, Variant},
};

use crate::resources::{
    traits::{CommonShape3D, ResourceId},
    utils::macros::resource_object,
};

use super::space::Space3D;

resource_object!(Area3D, area_create, PhysicsServer3D);

impl Area3D {
    pub fn add_shape(
        &self,
        shape: impl CommonShape3D,
        transform: Transform3D,
        disabled: bool,
    ) -> &Self {
        PhysicsServer3D::singleton()
            .area_add_shape_ex(self.get_rid(), shape.get_rid())
            .transform(transform)
            .disabled(disabled)
            .done();
        self
    }
    pub fn attach_object_instnace_id(&self, id: u64) -> &Self {
        PhysicsServer3D::singleton().area_attach_object_instance_id(self.get_rid(), id);
        self
    }
    pub fn clear_shapes(&self) -> &Self {
        PhysicsServer3D::singleton().area_clear_shapes(self.get_rid());
        self
    }
    pub fn get_collision_layer(&self) -> u32 {
        PhysicsServer3D::singleton().area_get_collision_layer(self.get_rid())
    }
    pub fn get_collision_mask(&self) -> u32 {
        PhysicsServer3D::singleton().area_get_collision_mask(self.get_rid())
    }
    pub fn get_object_instance_id(&self) -> u64 {
        PhysicsServer3D::singleton().area_get_object_instance_id(self.get_rid())
    }
    pub fn get_param(&self, param: AreaParameter) -> Variant {
        PhysicsServer3D::singleton().area_get_param(self.get_rid(), param)
    }
    pub fn get_shape(&self, shape_idx: i32) -> Rid {
        PhysicsServer3D::singleton().area_get_shape(self.get_rid(), shape_idx)
    }
    pub fn get_shape_count(&self) -> i32 {
        PhysicsServer3D::singleton().area_get_shape_count(self.get_rid())
    }
    pub fn get_shape_transform(&self, shape_idx: i32) -> Transform3D {
        PhysicsServer3D::singleton().area_get_shape_transform(self.get_rid(), shape_idx)
    }
    pub fn get_space(&self) -> Space3D {
        Space3D::from_rid(PhysicsServer3D::singleton().area_get_space(self.get_rid()))
    }
    pub fn get_transform(&self) -> Transform3D {
        PhysicsServer3D::singleton().area_get_transform(self.get_rid())
    }
    pub fn remove_shape(&self, shape_idx: i32) -> &Self {
        PhysicsServer3D::singleton().area_remove_shape(self.get_rid(), shape_idx);
        self
    }
    pub fn set_area_monitor_callback(&self, callback: Callable) -> &Self {
        PhysicsServer3D::singleton().area_set_area_monitor_callback(self.get_rid(), callback);
        self
    }
    pub fn set_collision_layer(&self, layer: u32) -> &Self {
        PhysicsServer3D::singleton().area_set_collision_layer(self.get_rid(), layer);
        self
    }
    pub fn set_collision_mask(&self, mask: u32) -> &Self {
        PhysicsServer3D::singleton().area_set_collision_mask(self.get_rid(), mask);
        self
    }
    pub fn set_monitor_callback(&self, callback: Callable) -> &Self {
        PhysicsServer3D::singleton().area_set_monitor_callback(self.get_rid(), callback);
        self
    }
    pub fn set_monitorable(&self, monitorable: bool) -> &Self {
        PhysicsServer3D::singleton().area_set_monitorable(self.get_rid(), monitorable);
        self
    }
    pub fn set_param(&self, param: AreaParameter, value: Variant) -> &Self {
        PhysicsServer3D::singleton().area_set_param(self.get_rid(), param, value);
        self
    }
    pub fn set_ray_pickable(&self, enable: bool) -> &Self {
        PhysicsServer3D::singleton().area_set_ray_pickable(self.get_rid(), enable);
        self
    }
    pub fn set_shape(&self, shape_idx: i32, shape: impl CommonShape3D) -> &Self {
        PhysicsServer3D::singleton().area_set_shape(self.get_rid(), shape_idx, shape.get_rid());
        self
    }
    pub fn set_shape_disabled(&self, shape_idx: i32, disabled: bool) -> &Self {
        PhysicsServer3D::singleton().area_set_shape_disabled(self.get_rid(), shape_idx, disabled);
        self
    }
    pub fn set_shape_transform(&self, shape_idx: i32, transform: Transform3D) -> &Self {
        PhysicsServer3D::singleton().area_set_shape_transform(self.get_rid(), shape_idx, transform);
        self
    }
    pub fn set_space(&self, space: Space3D) -> &Self {
        PhysicsServer3D::singleton().area_set_space(self.get_rid(), space.get_rid());
        self
    }
    pub fn set_transform(&self, transform: Transform3D) -> &Self {
        PhysicsServer3D::singleton().area_set_transform(self.get_rid(), transform);
        self
    }
}
