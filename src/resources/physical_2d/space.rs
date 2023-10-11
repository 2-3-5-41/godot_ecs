use crate::resources::{traits::ResourceId, utils::macros::resource_object};
use godot::{
    engine::{physics_server_2d::SpaceParameter, PhysicsDirectSpaceState2D, PhysicsServer2D},
    prelude::{Gd, Rid},
};

resource_object!(Space2D, space_create, PhysicsServer2D);

impl Space2D {
    pub fn get_direct_state(&self) -> Option<Gd<PhysicsDirectSpaceState2D>> {
        PhysicsServer2D::singleton().space_get_direct_state(self.get_rid())
    }
    pub fn get_param(&self, param: SpaceParameter) -> f32 {
        PhysicsServer2D::singleton().space_get_param(self.get_rid(), param)
    }
    pub fn is_active(&self) -> bool {
        PhysicsServer2D::singleton().space_is_active(self.get_rid())
    }
    pub fn set_active(&self, active: bool) -> &Self {
        PhysicsServer2D::singleton().space_set_active(self.get_rid(), active);
        self
    }
    pub fn set_param(&self, param: SpaceParameter, value: f32) -> &Self {
        PhysicsServer2D::singleton().space_set_param(self.get_rid(), param, value);
        self
    }
}
