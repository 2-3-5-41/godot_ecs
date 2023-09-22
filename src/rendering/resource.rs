use bevy_ecs::{system::Resource, world::FromWorld};
use bevy_utils::{hashbrown::HashMap, Uuid};
use godot::{
    engine::{image::Format, rendering_server::TextureLayeredType, Image, RenderingServer},
    prelude::{godot_error, Array, Dictionary, Gd, Rid},
};
pub enum RendererCreate {
    CameraAttributes,
    Camera,
    Canvas,
    CanvasItem,
    CabvasLight,
    CanvasLightOccluder,
    CavnasOccluderPolygon,
    CanvasTexture,
    Decal,
    DirectionalLight,
    Environment,
    FogVolume,
    Instance,
    Lightmap,
    Material,
    Mesh,
    MeshFromSurfaces(Array<Dictionary>, i32),
    Multimesh,
    Occluder,
    OmniLight,
    ParticlesCollision,
    Particles,
    ReflectionProbe,
    Scenario,
    Shader,
    Skeleton,
    Sky,
    SpotLight,
    Texture2d(Gd<Image>),
    Texture2dLayered(Array<Gd<Image>>, TextureLayeredType),
    Texture2dLayeredPlaceholder(TextureLayeredType),
    Texture2dPlaceholder,
    Texture3d(Format, i32, i32, i32, bool, Array<Gd<Image>>),
    Texture3dPlaceholder,
    TextureProxy(Rid),
    Viewport,
    VisibilityNotifier,
    VoxelGI,
}

impl RendererCreate {}

/// A struct that stores a valid [`Rid`] that it was able to create
/// on the [`RenderingServer`], or a provided pre-existing valid [`Rid`].
#[derive(Debug, Clone, Copy)]
pub struct RendererResource(Option<u64>);

impl RendererResource {
    /// Creates a new [`RendererResource`] based on what [`RendererCreate`]
    /// enum is passed through, this also creates the corresponding [`Rid`] on the
    /// [`RenderingServer`] for Godot.
    pub fn create(create: RendererCreate) -> Self {
        match create {
            RendererCreate::CameraAttributes => Self(
                RenderingServer::singleton()
                    .camera_attributes_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Camera => {
                Self(RenderingServer::singleton().camera_create().to_valid_u64())
            }
            RendererCreate::Canvas => {
                Self(RenderingServer::singleton().canvas_create().to_valid_u64())
            }
            RendererCreate::CanvasItem => Self(
                RenderingServer::singleton()
                    .canvas_item_create()
                    .to_valid_u64(),
            ),
            RendererCreate::CabvasLight => Self(
                RenderingServer::singleton()
                    .canvas_light_create()
                    .to_valid_u64(),
            ),
            RendererCreate::CanvasLightOccluder => Self(
                RenderingServer::singleton()
                    .canvas_light_occluder_create()
                    .to_valid_u64(),
            ),
            RendererCreate::CavnasOccluderPolygon => Self(
                RenderingServer::singleton()
                    .canvas_occluder_polygon_create()
                    .to_valid_u64(),
            ),
            RendererCreate::CanvasTexture => Self(
                RenderingServer::singleton()
                    .canvas_texture_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Decal => {
                Self(RenderingServer::singleton().decal_create().to_valid_u64())
            }
            RendererCreate::DirectionalLight => Self(
                RenderingServer::singleton()
                    .directional_light_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Environment => Self(
                RenderingServer::singleton()
                    .environment_create()
                    .to_valid_u64(),
            ),
            RendererCreate::FogVolume => Self(
                RenderingServer::singleton()
                    .fog_volume_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Instance => Self(
                RenderingServer::singleton()
                    .instance_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Lightmap => Self(
                RenderingServer::singleton()
                    .lightmap_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Material => Self(
                RenderingServer::singleton()
                    .material_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Mesh => Self(RenderingServer::singleton().mesh_create().to_valid_u64()),
            RendererCreate::MeshFromSurfaces(surfaces, blend_shape_count) => {
                let rid = RenderingServer::singleton()
                    .mesh_create_from_surfaces_ex(surfaces)
                    .blend_shape_count(blend_shape_count)
                    .done()
                    .to_valid_u64();
                Self(rid)
            }
            RendererCreate::Multimesh => Self(
                RenderingServer::singleton()
                    .multimesh_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Occluder => Self(
                RenderingServer::singleton()
                    .occluder_create()
                    .to_valid_u64(),
            ),
            RendererCreate::OmniLight => Self(
                RenderingServer::singleton()
                    .omni_light_create()
                    .to_valid_u64(),
            ),
            RendererCreate::ParticlesCollision => Self(
                RenderingServer::singleton()
                    .particles_collision_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Particles => Self(
                RenderingServer::singleton()
                    .particles_create()
                    .to_valid_u64(),
            ),
            RendererCreate::ReflectionProbe => Self(
                RenderingServer::singleton()
                    .reflection_probe_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Scenario => Self(
                RenderingServer::singleton()
                    .scenario_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Shader => {
                Self(RenderingServer::singleton().shader_create().to_valid_u64())
            }
            RendererCreate::Skeleton => Self(
                RenderingServer::singleton()
                    .skeleton_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Sky => Self(RenderingServer::singleton().sky_create().to_valid_u64()),
            RendererCreate::SpotLight => Self(
                RenderingServer::singleton()
                    .spot_light_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Texture2d(image) => Self(
                RenderingServer::singleton()
                    .texture_2d_create(image)
                    .to_valid_u64(),
            ),
            RendererCreate::Texture2dLayered(layers, layered_type) => Self(
                RenderingServer::singleton()
                    .texture_2d_layered_create(layers, layered_type)
                    .to_valid_u64(),
            ),
            RendererCreate::Texture2dLayeredPlaceholder(layered_type) => Self(
                RenderingServer::singleton()
                    .texture_2d_layered_placeholder_create(layered_type)
                    .to_valid_u64(),
            ),
            RendererCreate::Texture2dPlaceholder => Self(
                RenderingServer::singleton()
                    .texture_2d_placeholder_create()
                    .to_valid_u64(),
            ),
            RendererCreate::Texture3d(format, width, height, depth, mipmaps, data) => Self(
                RenderingServer::singleton()
                    .texture_3d_create(format, width, height, depth, mipmaps, data)
                    .to_valid_u64(),
            ),
            RendererCreate::Texture3dPlaceholder => Self(
                RenderingServer::singleton()
                    .texture_3d_placeholder_create()
                    .to_valid_u64(),
            ),
            RendererCreate::TextureProxy(base) => Self(
                RenderingServer::singleton()
                    .texture_proxy_create(base)
                    .to_valid_u64(),
            ),
            RendererCreate::Viewport => Self(
                RenderingServer::singleton()
                    .viewport_create()
                    .to_valid_u64(),
            ),
            RendererCreate::VisibilityNotifier => Self(
                RenderingServer::singleton()
                    .visibility_notifier_create()
                    .to_valid_u64(),
            ),
            RendererCreate::VoxelGI => Self(
                RenderingServer::singleton()
                    .voxel_gi_create()
                    .to_valid_u64(),
            ),
        }
    }
    pub fn try_get_rid(&self) -> Option<u64> {
        self.0.clone()
    }
    pub fn get_rid(&self) -> u64 {
        self.try_get_rid().unwrap()
    }
    pub fn free(&self) {
        RenderingServer::singleton().free_rid(Rid::new(self.0.unwrap()));
    }
    /// Creates a [`RendererResource`] if the provided pre-existing
    /// [`Rid`] is valid, and if not, returns [`None`].
    pub fn from_existing(rid: Option<u64>) -> Option<Self> {
        match rid {
            Some(_) => Some(Self(rid)),
            None => {
                godot_error!("Could not create a `RendererResource` with an `Invalid` Rid!");
                None
            }
        }
    }
}

/// Similar to bevy's asset resource, this will hold all [`RendererResource`]s
/// that can be accessed via the generated Uuid.
#[derive(Resource, Clone, Debug)]
pub struct RenderingServerResources(HashMap<Uuid, RendererResource>);

impl RenderingServerResources {
    pub fn new() -> Self {
        Self(HashMap::default())
    }
    pub fn insert(&mut self, resource: RendererResource) -> Option<Uuid> {
        let uuid = Uuid::new_v4();
        self.0.insert(uuid, resource);
        Some(uuid)
    }
    pub fn get_resource(&mut self, uuid: Uuid) -> Option<&RendererResource> {
        self.0.get(&uuid)
    }
    pub fn get_resource_mut(&mut self, uuid: Uuid) -> Option<&mut RendererResource> {
        self.0.get_mut(&uuid)
    }
    pub fn free_all(&mut self) {
        self.0.drain().for_each(|(_, resource)| {
            RenderingServer::singleton().free_rid(Rid::new(resource.get_rid()))
        });
    }
}
