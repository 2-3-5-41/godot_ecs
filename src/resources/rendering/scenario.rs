use godot::{engine::RenderingServer, prelude::Rid};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

use super::{camera_attributes::CameraAttributes, environment::Environment};

resource_object!(Scenario, scenario_create, RenderingServer);

impl Scenario {
    pub fn set_camera_attributes(&self, effects: CameraAttributes) -> &Self {
        RenderingServer::singleton()
            .scenario_set_camera_attributes(self.get_rid(), effects.get_rid());
        self
    }
    pub fn set_environment(&self, environment: Environment) -> &Self {
        RenderingServer::singleton()
            .scenario_set_environment(self.get_rid(), environment.get_rid());
        self
    }
    pub fn set_fallback_environment(&self, environment: Environment) -> &Self {
        RenderingServer::singleton()
            .scenario_set_fallback_environment(self.get_rid(), environment.get_rid());
        self
    }
}
