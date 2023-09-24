use std::collections::HashMap;

use bevy_ecs::{component::Component, system::Resource};
use bevy_utils::Uuid;
use godot::engine::RenderingServer;

use super::ResourceId;

#[derive(Resource, Debug, Clone)]
pub struct RidServer<R: ResourceId> {
    hash_map: HashMap<Uuid, R>,
}

impl<R: ResourceId> RidServer<R> {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::default(),
        }
    }
    pub fn add(&mut self, resource: R) -> ResourceHandle {
        let uuid = Uuid::new_v4();
        self.hash_map.insert(uuid, resource);
        ResourceHandle::Valid(uuid)
    }
    pub fn try_get(&self, handle: &ResourceHandle) -> Option<&R> {
        if let ResourceHandle::Valid(uuid) = handle {
            self.hash_map.get(uuid)
        } else {
            panic!("Invalid resource handle!")
        };

        None
    }
    pub fn free_all(&mut self) {
        self.hash_map
            .drain()
            .for_each(|(_, resource)| RenderingServer::singleton().free_rid(resource.get_rid()))
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ResourceHandle {
    Valid(Uuid),
    Invalid,
}
