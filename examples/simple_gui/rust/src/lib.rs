use godot::{prelude::*, engine::{CanvasGroupVirtual, TextServerManager, Control}};
use godot_ecs::ecs::Ecs;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SimpleGUI {
    ecs: Ecs,
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl CanvasGroupVirtual for SimpleGUI {
    fn init(base: Base<Control>) -> Self {
        let ecs = Ecs::default();
        Self { ecs, base }
    }
    fn ready(&mut self) {
        godot_print!("{:?}", TextServerManager::singleton().get_interface(1))
    }
}

#[godot_api]
impl SimpleGUI {}