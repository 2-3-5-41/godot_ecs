use godot::{engine::PhysicsServer3D, prelude::Rid};

use super::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub enum Physical3dType {
    Area(Rid),
    Body(Rid),
    BoxShape(Rid),
    CapsuleShape(Rid),
    ConcavePolygonShape(Rid),
    ConvexPolygonShape(Rid),
    CustomShape(Rid),
    CylinderShape(Rid),
    HeightmapShape(Rid),
    Joint(Rid),
    SeparationRayShape(Rid),
    Space(Rid),
    SphereShape(Rid),
    WorldBoundaryShape(Rid),
}

impl Into<Rid> for Physical3dType {
    fn into(self) -> Rid {
        match self {
            Physical3dType::Area(rid) => rid,
            Physical3dType::Body(rid) => rid,
            Physical3dType::BoxShape(rid) => rid,
            Physical3dType::CapsuleShape(rid) => rid,
            Physical3dType::ConcavePolygonShape(rid) => rid,
            Physical3dType::ConvexPolygonShape(rid) => rid,
            Physical3dType::CustomShape(rid) => rid,
            Physical3dType::CylinderShape(rid) => rid,
            Physical3dType::HeightmapShape(rid) => rid,
            Physical3dType::Joint(rid) => rid,
            Physical3dType::SeparationRayShape(rid) => rid,
            Physical3dType::Space(rid) => rid,
            Physical3dType::SphereShape(rid) => rid,
            Physical3dType::WorldBoundaryShape(rid) => rid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Physical3D(Physical3dType);

impl ResourceId for Physical3D {
    fn get_rid(&self) -> Rid {
        self.0.into()
    }

    fn free_rid(&self) {
        PhysicsServer3D::singleton().free_rid(self.get_rid())
    }
}

impl Physical3D {
    pub fn new(resource: Physical3dType) -> Self {
        Self(resource)
    }
    pub fn get_type(&self) -> &Physical3dType {
        &self.0
    }
}
