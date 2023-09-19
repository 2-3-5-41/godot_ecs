use bevy::reflect::TypeUuid;
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

/// A struct that stores a valid [`Rid`] that it was able to create
/// on the [`RenderingServer`], or a provided pre-existing valid [`Rid`].
#[derive(TypeUuid, Clone, Copy, Debug)]
#[uuid = "5304752c-4058-41e9-8231-e7031d8e6651"]
pub struct RendererResource(Rid);

impl RendererResource {
    /// Creates a new [`RendererResource`] based on what [`RendererCreate`]
    /// enum is passed through, this also creates the corresponding [`Rid`] on the
    /// [`RenderingServer`] for Godot.
    pub fn create(create: RendererCreate) -> Self {
        match create {
            RendererCreate::CameraAttributes => {
                Self(RenderingServer::singleton().camera_attributes_create())
            }
            RendererCreate::Camera => Self(RenderingServer::singleton().camera_create()),
            RendererCreate::Canvas => Self(RenderingServer::singleton().canvas_create()),
            RendererCreate::CanvasItem => Self(RenderingServer::singleton().canvas_item_create()),
            RendererCreate::CabvasLight => Self(RenderingServer::singleton().canvas_light_create()),
            RendererCreate::CanvasLightOccluder => {
                Self(RenderingServer::singleton().canvas_light_occluder_create())
            }
            RendererCreate::CavnasOccluderPolygon => {
                Self(RenderingServer::singleton().canvas_occluder_polygon_create())
            }
            RendererCreate::CanvasTexture => {
                Self(RenderingServer::singleton().canvas_texture_create())
            }
            RendererCreate::Decal => Self(RenderingServer::singleton().decal_create()),
            RendererCreate::DirectionalLight => {
                Self(RenderingServer::singleton().directional_light_create())
            }
            RendererCreate::Environment => Self(RenderingServer::singleton().environment_create()),
            RendererCreate::FogVolume => Self(RenderingServer::singleton().fog_volume_create()),
            RendererCreate::Instance => Self(RenderingServer::singleton().instance_create()),
            RendererCreate::Lightmap => Self(RenderingServer::singleton().lightmap_create()),
            RendererCreate::Material => Self(RenderingServer::singleton().material_create()),
            RendererCreate::Mesh => Self(RenderingServer::singleton().mesh_create()),
            RendererCreate::MeshFromSurfaces(surfaces, blend_shape_count) => {
                let rid = RenderingServer::singleton()
                    .mesh_create_from_surfaces_ex(surfaces)
                    .blend_shape_count(blend_shape_count)
                    .done();
                Self(rid)
            }
            RendererCreate::Multimesh => Self(RenderingServer::singleton().multimesh_create()),
            RendererCreate::Occluder => Self(RenderingServer::singleton().occluder_create()),
            RendererCreate::OmniLight => Self(RenderingServer::singleton().omni_light_create()),
            RendererCreate::ParticlesCollision => {
                Self(RenderingServer::singleton().particles_collision_create())
            }
            RendererCreate::Particles => Self(RenderingServer::singleton().particles_create()),
            RendererCreate::ReflectionProbe => {
                Self(RenderingServer::singleton().reflection_probe_create())
            }
            RendererCreate::Scenario => Self(RenderingServer::singleton().scenario_create()),
            RendererCreate::Shader => Self(RenderingServer::singleton().shader_create()),
            RendererCreate::Skeleton => Self(RenderingServer::singleton().skeleton_create()),
            RendererCreate::Sky => Self(RenderingServer::singleton().sky_create()),
            RendererCreate::SpotLight => Self(RenderingServer::singleton().spot_light_create()),
            RendererCreate::Texture2d(image) => {
                Self(RenderingServer::singleton().texture_2d_create(image))
            }
            RendererCreate::Texture2dLayered(layers, layered_type) => {
                Self(RenderingServer::singleton().texture_2d_layered_create(layers, layered_type))
            }
            RendererCreate::Texture2dLayeredPlaceholder(layered_type) => Self(
                RenderingServer::singleton().texture_2d_layered_placeholder_create(layered_type),
            ),
            RendererCreate::Texture2dPlaceholder => {
                Self(RenderingServer::singleton().texture_2d_placeholder_create())
            }
            RendererCreate::Texture3d(format, width, height, depth, mipmaps, data) => Self(
                RenderingServer::singleton()
                    .texture_3d_create(format, width, height, depth, mipmaps, data),
            ),
            RendererCreate::Texture3dPlaceholder => {
                Self(RenderingServer::singleton().texture_3d_placeholder_create())
            }
            RendererCreate::TextureProxy(base) => {
                Self(RenderingServer::singleton().texture_proxy_create(base))
            }
            RendererCreate::Viewport => Self(RenderingServer::singleton().viewport_create()),
            RendererCreate::VisibilityNotifier => {
                Self(RenderingServer::singleton().visibility_notifier_create())
            }
            RendererCreate::VoxelGI => Self(RenderingServer::singleton().voxel_gi_create()),
        }
    }
    pub fn get_rid(&self) -> Rid {
        self.0.clone()
    }
    pub fn free(&self) {
        RenderingServer::singleton().free_rid(self.0);
    }
    /// Creates a [`RendererResource`] if the provided pre-existing
    /// [`Rid`] is valid, and if not, returns [`None`].
    pub fn from_existing(rid: Rid) -> Option<Self> {
        match rid {
            Rid::Valid(_) => Some(Self(rid)),
            Rid::Invalid => {
                godot_error!("Could not create a `RendererResource` with an `Invalid` Rid!");
                None
            }
        }
    }
}
