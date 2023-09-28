use bevy_ecs::system::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct DeltaTime(pub f64);

impl DeltaTime {
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}
