use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};
use godot_ecs::ecs::Ecs;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base = Control)]
pub struct ImportTest {
    ecs: Ecs,
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for ImportTest {
    fn init(base: Base<Control>) -> Self {
        let ecs = Ecs::default();
        Self { ecs, base }
    }
}

#[godot_api]
impl ImportTest {}
