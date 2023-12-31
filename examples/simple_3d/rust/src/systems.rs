use bevy_ecs::{
    query::With,
    system::{Commands, Query, Res, ResMut},
};
use godot::{
    engine::rendering_server::{LightDirectionalShadowMode, LightDirectionalSkyMode, LightParam},
    prelude::{Basis, Transform3D, Vector3},
};
use godot_ecs::resources::{
    rendering::{
        camera::Camera, directional_light::DirectionalLight, instance::Instance,
        scenario::Scenario, viewport::Viewport,
    },
    rid_server::{ResourceHandle, RidServer},
    traits::{CommonLight3D, ResourceId},
};

use crate::{
    components::{MainCamera, MainScenario, MainViewport, Position, Sun, Velocity},
    resources::DeltaTime,
};

pub fn init(
    viewports: Res<RidServer<Viewport>>,
    viewport_query: Query<&ResourceHandle, With<MainViewport>>,
    mut cameras: ResMut<RidServer<Camera>>,
    mut directional_lights: ResMut<RidServer<DirectionalLight>>,
    scenarios: Res<RidServer<Scenario>>,
    scenario_query: Query<&ResourceHandle, With<MainScenario>>,
    mut instances: ResMut<RidServer<Instance>>,
    mut commands: Commands,
) {
    let scenario = scenarios.get(scenario_query.get_single().unwrap());

    // Let's initialize our main camera.
    let main_camera = Camera::create();
    main_camera
        .set_perspective(70.0, 0.1, 4000.0)
        .set_transform(
            Transform3D::new(Basis::IDENTITY, Vector3::new(1.0, 1.0, -5.0)).looking_at(
                Vector3::ZERO,
                Vector3::UP,
                false,
            ),
        );

    // Now we can attach our camera to the main viewport.
    viewports
        .get(viewport_query.get_single().unwrap())
        .attach_camera(&main_camera);

    // Now we can create a sun, first, we need to create an `Instance` so we can render, and position the sun in 3D space.
    let sun_instance = Instance::create();

    // Then we create our sun, and initialize all our sun's parameters.
    // The parameters used here are based on what's used in Godot's source code; `scene/3d/light_3d.cpp`.
    let sun_light = DirectionalLight::create();

    sun_light
        .set_shadow(true)
        .set_shadow_mode(LightDirectionalShadowMode::LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS)
        .set_sky_mode(LightDirectionalSkyMode::LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY)
        .set_param(LightParam::LIGHT_PARAM_SHADOW_MAX_DISTANCE, 100.0)
        .set_param(LightParam::LIGHT_PARAM_SHADOW_FADE_START, 0.8)
        .set_param(LightParam::LIGHT_PARAM_SHADOW_NORMAL_BIAS, 2.0)
        .set_param(LightParam::LIGHT_PARAM_INTENSITY, 100000.0);

    // Then we add our sun to the instance, and the scenario.
    sun_instance
        .set_base(sun_light.get_rid())
        .set_scenario(scenario)
        .set_transform(
            Transform3D::new(Basis::IDENTITY, Vector3::new(-10.0, 3.0, -5.0)).looking_at(
                Vector3::ZERO,
                Vector3::UP,
                false,
            ),
        )
        .set_visible(true);

    // Finally, to maintain access to this data, we should add them to entities in the ECS.
    commands.spawn((
        cameras.add(main_camera),
        Position(Vector3::new(1.0, 1.0, -5.0)),
        Velocity(Vector3::new(0.2, 0.0, 0.0)),
        MainCamera,
    ));
    commands.spawn(directional_lights.add(sun_light));
    commands.spawn((
        instances.add(sun_instance),
        Position(Vector3::new(-10.0, 3.0, -5.0)),
        Velocity(Vector3::new(0.0, 0.0, 1.0)),
        Sun,
    ));
}

pub fn move_sun(
    delta: Res<DeltaTime>,
    suns: Res<RidServer<Instance>>,
    mut query: Query<(&ResourceHandle, &mut Position, &Velocity), With<Sun>>,
) {
    query.iter_mut().for_each(|(res, mut pos, vel)| {
        if pos.0.z >= 10.0 {
            return;
        }

        pos.0.z += vel.0.z * delta.as_f32();

        let sun = suns.get(res);

        sun.set_transform(
            Transform3D::new(Basis::IDENTITY, Vector3::new(pos.0.x, pos.0.y, pos.0.z)).looking_at(
                Vector3::ZERO,
                Vector3::UP,
                false,
            ),
        );
    });
}

pub fn move_camera(
    delta: Res<DeltaTime>,
    cameras: Res<RidServer<Camera>>,
    mut query: Query<(&ResourceHandle, &mut Position, &Velocity), With<MainCamera>>,
) {
    query.iter_mut().for_each(|(res, mut pos, vel)| {
        pos.0.x += vel.0.x * delta.as_f32();

        let camera = cameras.get(res);

        camera.set_transform(
            Transform3D::new(Basis::IDENTITY, Vector3::new(pos.0.x, pos.0.y, pos.0.z)).looking_at(
                Vector3::ZERO,
                Vector3::UP,
                false,
            ),
        );
    })
}
