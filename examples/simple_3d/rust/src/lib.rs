use godot::prelude::*;

mod components;
mod resources;
mod systems;
mod world;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
