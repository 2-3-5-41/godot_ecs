use godot::{engine::PhysicsServer2D, prelude::Rid};

use super::traits::ResourceId;

#[derive(Debug, Clone, Copy)]
pub enum Physical2dType {
    Area(Rid),
    Body(Rid),
    CapsuleShape(Rid),
    CircleShape(Rid),
    ConcavePolygonShape(Rid),
    ConvexPolygonShape(Rid),
    Joint(Rid),
    RectangleShape(Rid),
    SegmentShape(Rid),
    SeparationRayShape(Rid),
    Space(Rid),
    WorldBoundaryShape(Rid),
}

impl Into<Rid> for Physical2dType {
    fn into(self) -> Rid {
        match self {
            Physical2dType::Area(rid) => rid,
            Physical2dType::Body(rid) => rid,
            Physical2dType::CapsuleShape(rid) => rid,
            Physical2dType::CircleShape(rid) => rid,
            Physical2dType::ConcavePolygonShape(rid) => rid,
            Physical2dType::ConvexPolygonShape(rid) => rid,
            Physical2dType::Joint(rid) => rid,
            Physical2dType::RectangleShape(rid) => rid,
            Physical2dType::SegmentShape(rid) => rid,
            Physical2dType::SeparationRayShape(rid) => rid,
            Physical2dType::Space(rid) => rid,
            Physical2dType::WorldBoundaryShape(rid) => rid,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Physical2D(Physical2dType);

impl ResourceId for Physical2D {
    fn get_rid(&self) -> Rid {
        self.0.into()
    }

    fn free_rid(&self) {
        PhysicsServer2D::singleton().free_rid(self.get_rid())
    }
}

impl Physical2D {
    pub fn new(resource: Physical2dType) -> Self {
        Self(resource)
    }
    pub fn get_type(&self) -> &Physical2dType {
        &self.0
    }
}
