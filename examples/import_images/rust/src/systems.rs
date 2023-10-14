use bevy_ecs::{
    query::{Added, With},
    system::{Commands, Query, Res, ResMut},
};
use godot::prelude::{Color, Rect2, Vector2};
use godot_ecs::resources::{
    rendering::{canvas_item::CanvasItem, texture_2d::Texture2D},
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
        let texture = textures.get(image);
        let texture_size = texture.get_image().get_size();
        let canvas_item = CanvasItem::create();

        // Create random position.
        let mut rng = rand::thread_rng();
        let position: (f32, f32) = rng.gen();

        // Create `Rect2` for our canvas item.
        let rect = Rect2::new(
            Vector2::new(position.0 * 512.0, position.1 * 512.0),
            Vector2::new(texture_size.x as f32, texture_size.y as f32),
        );

        canvas_item
            .add_texture_rect(rect, texture.get_rid(), false, Color::WHITE, false)
            .set_parent(root_canvas_item);

        let handle = canvas_items.add(canvas_item);

        commands.spawn((handle, CanvasItemComponent));
    }
}
