use components::{ImageImport, MainCanvas, RootCanvasItem};
use godot::{
    engine::{Control, ControlVirtual, Image},
    prelude::*,
};
use godot_ecs::{
    ecs::Ecs,
    godot_schedule::Process,
    resources::{
        renderable::{canvas::Canvas, canvas_item::CanvasItem, texture_2d::Texture2D},
        rid_server::RidServer,
    },
};

mod components;
mod systems;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base = Control)]
pub struct ImportTest {
    image_store: Array<Gd<Image>>,
    ecs: Ecs,
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for ImportTest {
    fn init(base: Base<Control>) -> Self {
        // Image store init
        let image_store = Array::<Gd<Image>>::new();

        // Ecs init
        let mut ecs = Ecs::default();

        ecs.insert_resource(RidServer::<Texture2D>::new())
            .insert_resource(RidServer::<Canvas>::new())
            .insert_resource(RidServer::<CanvasItem>::new());

        ecs.add_systems(Process, systems::check_textures);

        // Our return struct
        Self {
            image_store,
            ecs,
            base,
        }
    }
    fn enter_tree(&mut self) {
        self.store_main_canvas();
        self.store_root_canvas_item();
    }
    fn process(&mut self, _delta: f64) {
        self.ecs.run_schedule(Process);
    }
}

#[godot_api]
impl ImportTest {
    #[func]
    pub fn on_files_dropped(&mut self, files: PackedStringArray) {
        for file in files.as_slice() {
            let Some(new_image) = Image::load_from_file(file.to_owned()) else {
                godot_error!("Failed to load image from {file:?}!");
                continue;
            };

            let texture = Texture2D::create(
                self.store_image(new_image)
                    .expect("Failed to store and retrieve new image!"),
            );

            self.add_texture_to_world(texture);
        }
    }
    fn store_image(&mut self, image: Gd<Image>) -> Option<Gd<Image>> {
        self.image_store.push(image);
        let image = self.image_store.last();

        image
    }
    fn add_texture_to_world(&mut self, texture: Texture2D) {
        let world = self.ecs.get_world_mut();
        let handle = world.resource_mut::<RidServer<Texture2D>>().add(texture);

        world.spawn((handle, ImageImport));
    }
    fn store_main_canvas(&mut self) {
        let world = self.ecs.get_world_mut();
        let mut canvas_server = world.resource_mut::<RidServer<Canvas>>();
        let main_canvas = Canvas::from_rid(self.base.get_canvas());
        let handle = canvas_server.add(main_canvas);

        world.spawn((handle, MainCanvas));
    }
    fn store_root_canvas_item(&mut self) {
        let world = self.ecs.get_world_mut();
        let mut canvas_server = world.resource_mut::<RidServer<CanvasItem>>();
        let root_canvas_item = CanvasItem::from_rid(self.base.get_canvas_item());
        let handle = canvas_server.add(root_canvas_item);

        world.spawn((handle, RootCanvasItem));
    }
}
