use godot::{prelude::*, engine::RenderingServer};

use super::ResourceId;

#[derive(Debug, Clone, Copy)]
pub enum RenderableType {
    CameraAttributes(Rid),
    Camera(Rid),
    Canvas(Rid),
    CanvasItem(Rid),
    CanvasLight(Rid),
    CanvasLightOccluder(Rid),
    CavnasOccluderPolygon(Rid),
    CanvasTexture(Rid),
    Decal(Rid),
    DirectionalLight(Rid),
    Environment(Rid),
    FogVolume(Rid),
    Instance(Rid),
    Lightmap(Rid),
    Material(Rid),
    Mesh(Rid),
    MeshFromSurfaces(Rid),
    Multimesh(Rid),
    Occluder(Rid),
    OmniLight(Rid),
    ParticlesCollision(Rid),
    Particles(Rid),
    ReflectionProbe(Rid),
    Scenario(Rid),
    Shader(Rid),
    Skeleton(Rid),
    Sky(Rid),
    SpotLight(Rid),
    Texture2d(Rid),
    Texture2dLayered(Rid),
    Texture2dLayeredPlaceholder(Rid),
    Texture2dPlaceholder(Rid),
    Texture3d(Rid),
    Texture3dPlaceholder(Rid),
    TextureProxy(Rid),
    Viewport(Rid),
    VisibilityNotifier(Rid),
    VoxelGI(Rid),
}

impl Into<Rid> for RenderableType {
    fn into(self) -> Rid {
        match self {
            RenderableType::CameraAttributes(rid) => rid,
            RenderableType::Camera(rid) => rid,
            RenderableType::Canvas(rid) => rid,
            RenderableType::CanvasItem(rid) => rid,
            RenderableType::CanvasLight(rid) => rid,
            RenderableType::CanvasLightOccluder(rid) => rid,
            RenderableType::CavnasOccluderPolygon(rid) => rid,
            RenderableType::CanvasTexture(rid) => rid,
            RenderableType::Decal(rid) => rid,
            RenderableType::DirectionalLight(rid) => rid,
            RenderableType::Environment(rid) => rid,
            RenderableType::FogVolume(rid) => rid,
            RenderableType::Instance(rid) => rid,
            RenderableType::Lightmap(rid) => rid,
            RenderableType::Material(rid) => rid,
            RenderableType::Mesh(rid) => rid,
            RenderableType::MeshFromSurfaces(rid) => rid,
            RenderableType::Multimesh(rid) => rid,
            RenderableType::Occluder(rid) => rid,
            RenderableType::OmniLight(rid) => rid,
            RenderableType::ParticlesCollision(rid) => rid,
            RenderableType::Particles(rid) => rid,
            RenderableType::ReflectionProbe(rid) => rid,
            RenderableType::Scenario(rid) => rid,
            RenderableType::Shader(rid) => rid,
            RenderableType::Skeleton(rid) => rid,
            RenderableType::Sky(rid) => rid,
            RenderableType::SpotLight(rid) => rid,
            RenderableType::Texture2d(rid) => rid,
            RenderableType::Texture2dLayered(rid) => rid,
            RenderableType::Texture2dLayeredPlaceholder(rid) => rid,
            RenderableType::Texture2dPlaceholder(rid) => rid,
            RenderableType::Texture3d(rid) => rid,
            RenderableType::Texture3dPlaceholder(rid) => rid,
            RenderableType::TextureProxy(rid) => rid,
            RenderableType::Viewport(rid) => rid,
            RenderableType::VisibilityNotifier(rid) => rid,
            RenderableType::VoxelGI(rid) => rid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Renderable(RenderableType);

impl ResourceId for Renderable {
    fn get_rid(&self) -> Rid {
        self.0.into()
    }
    fn free_rid(&self) {
        RenderingServer::singleton().free_rid(self.get_rid())
    }
}

impl Renderable {
    pub fn new(resource: RenderableType) -> Self {
        Self(resource)
    }
    pub fn get_type(&self) -> &RenderableType {
        &self.0
    }
}