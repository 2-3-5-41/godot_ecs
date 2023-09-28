use bevy_ecs::system::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct DeltaTime(pub f64);
