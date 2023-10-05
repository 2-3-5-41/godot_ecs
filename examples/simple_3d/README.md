# [Example] Simple3D

This example will show you how you can create a simple 3d scene in Godot through [`bevy_ecs`](https://docs.rs/bevy_ecs/latest/bevy_ecs) and Godot's [`RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).

Since we are working with visual instances in this example, we make use of a `Node3D` as our entry point, and event loop for our ECS, this is so we can access the `World3D -> Scenario` that is created on the rendering server by the `Node3D` and render our ECS made visual instances like the `DirectionalLight`.

## Note
You'll notice that we are calling other schedules in the `_ready`, `_process`, `_physics_process`, and `_exit_tree` functions in the node. While we aren't using them in this example, they do show how to call other schedules and resources outside of `bevy_ecs` systems.

Another thing to take note of; we are using both Nodes and ECS objects to create this simple 3d scene, this is thanks to us simply making use of Godot's internal servers just like their nodes under the hood.