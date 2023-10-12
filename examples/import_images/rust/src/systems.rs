use bevy_ecs::{
    query::{Added, With},
    system::{Commands, Query, Res, ResMut},
};
use godot::prelude::{Color, Rect2, Vector2};
use godot_ecs::resources::{
    renderable::{canvas_item::CanvasItem, texture_2d::Texture2D},
    rid_server::{ResourceHandle, RidServer},
    traits::ResourceId,
};
use rand::Rng;

use crate::components::{CanvasItemComponent, ImageImport, RootCanvasItem};

pub fn check_textures(
    textures: Res<RidServer<Texture2D>>,
    mut canvas_items: ResMut<RidServer<CanvasItem>>,
    new_images: Query<&ResourceHandle, Added<ImageImport>>,
    root_canvas_item: Query<&ResourceHandle, With<RootCanvasItem>>,
    mut commands: Commands,
) {
    let root_canvas_item = canvas_items
        .get(
            root_canvas_item
                .get_single()
                .expect("Failed to get main canvas!"),
        )
        .get_rid();

    for image in &new_images {
        let canvas_item = CanvasItem::create();

        let mut rng = rand::thread_rng();
        let position: (f32, f32) = rng.gen();

        let rect = Rect2::new(
            Vector2::new(position.0 * 512.0, position.1 * 512.0),
            Vector2::new(512.0, 512.0),
        );
        let texture = textures.get(image).get_rid();

        canvas_item
            .add_texture_rect(rect, texture, false, Color::WHITE, false)
            .set_parent(root_canvas_item);

        let handle = canvas_items.add(canvas_item);

        commands.spawn((handle, CanvasItemComponent));
    }
}
