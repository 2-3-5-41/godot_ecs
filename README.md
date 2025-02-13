# godot_ecs
What if Godot and Bevy got married? Well, you'd get one interesting duo of data driven goodness.

> **This crate is not maintained anymore** ⚠️
>
> I haven't touched the code for long enough that the two main features of this code base have made major changes that ask for a massive re-write. I personally do not have interest in taking on that task due to being a lazy dev, but also, I'm not a fan of how my younger self approached this solution.
>
> I'll just leave this repo here as one of my many stepping stones in my adventures of learning Rust, and gdext.

## Purpose
This crate provides a code first, ECS approach to game/app development in Godot, taking full advantage of Godot's [`RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html), [`PhysicsServer2D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2d.html), [`PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html), etc...

Do note, this crate does not plan on providing any in-editor tools for creating pre-made scenes compatible with [`bevy_ecs`](https://docs.rs/bevy_ecs/latest/bevy_ecs), however, it's not outside the realm of possibility.
