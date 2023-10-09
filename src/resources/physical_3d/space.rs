use godot::{
    engine::{physics_server_3d::SpaceParameter, PhysicsDirectSpaceState3D, PhysicsServer3D},
    prelude::{Gd, Rid},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Space3D, space_create, PhysicsServer3D);

impl Space3D {
    pub fn get_direct_state(&self) -> Option<Gd<PhysicsDirectSpaceState3D>> {
        PhysicsServer3D::singleton().space_get_direct_state(self.get_rid())
    }
    pub fn get_param(&self, param: SpaceParameter) -> f32 {
        PhysicsServer3D::singleton().space_get_param(self.get_rid(), param)
    }
    pub fn is_active(&self) -> bool {
        PhysicsServer3D::singleton().space_is_active(self.get_rid())
    }
    pub fn set_active(&self, active: bool) -> &Self {
        PhysicsServer3D::singleton().space_set_active(self.get_rid(), active);
        self
    }
    pub fn set_param(&self, param: SpaceParameter, value: f32) -> &Self {
        PhysicsServer3D::singleton().space_set_param(self.get_rid(), param, value);
        self
    }
}
