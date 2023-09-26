use std::collections::HashMap;

use bevy_ecs::{component::Component, system::Resource};
use bevy_utils::Uuid;
use godot::engine::RenderingServer;

use super::ResourceId;

/// # RidServer
/// This is a bevy resource similar to the bevy_asset `Assets` resource,
/// but re-implemented and narrowed down to interact with Godot's [`Rid`].
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
    pub fn get(&self, handle: &ResourceHandle) -> &R {
        self.try_get(handle).expect("Failed to get resource!")
    }
    pub fn try_get(&self, handle: &ResourceHandle) -> Option<&R> {
        if let ResourceHandle::Valid(uuid) = handle {
            return self.hash_map.get(uuid);
        } else {
            return None;
        };
    }
    pub fn free_all(&mut self) {
        self.hash_map
            .drain()
            .for_each(|(_, resource)| RenderingServer::singleton().free_rid(resource.get_rid()))
    }
    pub fn free_resource(&mut self, handle: &ResourceHandle) -> Result<(), String> {
        let ResourceHandle::Valid(uuid) = handle else {
            return Err("Cannot free invalid resource handle!".into());
        };
        let Some(resource) = self.hash_map.remove(uuid) else {
            return Err("There is no resource under this Uuid, removing key from hashtable.".into());
        };

        // Free rid from Godot's server.
        resource.free_rid();

        Ok(())
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ResourceHandle {
    Valid(Uuid),
    Invalid,
}
