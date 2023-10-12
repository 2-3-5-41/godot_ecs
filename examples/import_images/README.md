# [Example] Import Images
This example showcases how you can import images by dropping them on the window and creating ECS based entities in [`bevy_ecs`](https://docs.rs/bevy_ecs/latest/bevy_ecs) and Godot's [`RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).

Since objects like `Gd<Image>` aren't thread safe becuase we created them as we import our image(s), it's best to create an internal storage for those (non-send & non-sync) `Gd<T>` objects on the node itself, and referenceing them to create `Texture2D` instances on the `RenderingServer`.