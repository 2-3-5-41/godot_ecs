# godot_ecs
What if Godot and Bevy got married? Well, you'd get one interesting duo of data driven goodness.

## In Development
> **This crate is not production ready** ⚠️
>
> This crate serves as an example, *for now*, as to how you can use [`bevy_ecs`](https://docs.rs/bevy_ecs/latest/bevy_ecs) as a means to directly interact with Godot through GlobalScope objects like the [`RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).

## Purpose
This crate provides a code first, ECS approach to game/app development in Godot, taking full advantage of Godot's [`RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html), [`PhysicsServer2D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html), [`PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html), etc...

Do note, this crate does not plan on providing any in-editor tools for creating pre-made scenes compatible with [`bevy_ecs`](https://docs.rs/bevy_ecs/latest/bevy_ecs), however, it's not outside the realm of possibility.
