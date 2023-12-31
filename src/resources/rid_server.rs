use std::collections::HashMap;

use bevy_ecs::{component::Component, system::Resource};
use bevy_utils::Uuid;
use godot::engine::RenderingServer;

use super::traits::ResourceId;

/// # RidServer
/// This is a bevy resource similar to the bevy_asset `Assets` resource,
/// but re-implemented and narrowed down to interact with Godot's [`Rid`].
///
/// If you do need the functionality of bevy's `Asset` resource, you can simply add
/// `bevy_asset` to your `Cargo.toml` and insert the `Asset` resource into your ecs world.
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
    pub fn get_mut(&mut self, handle: &ResourceHandle) -> &mut R {
        self.try_get_mut(handle).expect("Failed to get resource!")
    }
    pub fn try_get(&self, handle: &ResourceHandle) -> Option<&R> {
        if let ResourceHandle::Valid(uuid) = handle {
            return self.hash_map.get(uuid);
        } else {
            return None;
        };
    }
    pub fn try_get_mut(&mut self, handle: &ResourceHandle) -> Option<&mut R> {
        if let ResourceHandle::Valid(uuid) = handle {
            return self.hash_map.get_mut(uuid);
        } else {
            return None;
        }
    }
    pub fn free_all(&mut self) {
        self.hash_map
            .drain()
            .for_each(|(_, resource)| RenderingServer::singleton().free_rid(resource.get_rid()))
    }
    pub fn remove(&mut self, handle: &ResourceHandle) -> Result<(), String> {
        let ResourceHandle::Valid(uuid) = handle else {
            return Err("Cannot free invalid resource handle!".into());
        };
        let Some(resource) = self.hash_map.remove(uuid) else {
            return Err(
                "There is no resource under this Uuid, removing key from hashtable.".into(),
            );
        };

        resource.free_rid();

        Ok(())
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ResourceHandle {
    Valid(Uuid),
    Invalid,
}

impl ResourceHandle {
    pub fn get_valid(&self) -> &Uuid {
        self.try_get_valid().unwrap()
    }
    pub fn try_get_valid(&self) -> Option<&Uuid> {
        match self {
            Self::Valid(uuid) => Some(&uuid),
            Self::Invalid => None,
        }
    }
    pub fn is_valid(&self) -> bool {
        if let Self::Valid(_) = self {
            true
        } else {
            false
        }
    }
}
