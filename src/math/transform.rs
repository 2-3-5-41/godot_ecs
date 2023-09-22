use bevy_ecs::component::Component;
use godot::prelude::{Basis, Transform3D, Vector3};

#[derive(Component, Clone, Copy, Debug)]
pub struct GdTransform3D {
    gd_transform: Transform3D,
}

impl GdTransform3D {
    pub fn from_origin(origin: Vector3) -> Self {
        Self {
            gd_transform: Transform3D::new(Basis::IDENTITY, origin),
        }
    }
    pub fn from_basis(basis: Basis) -> Self {
        Self {
            gd_transform: Transform3D::new(basis, Vector3::ZERO),
        }
    }
    pub fn get_transform(&self) -> &Transform3D {
        &self.gd_transform
    }
    pub fn get_transform_mut(&mut self) -> &mut Transform3D {
        &mut self.gd_transform
    }
    pub fn set_origin(&mut self, new: Vector3) -> &mut Self {
        self.get_transform_mut().origin = new;
        self
    }
    pub fn translate(&mut self, offset: Vector3) -> &mut Self {
        let _ = self.get_transform().translated(offset);
        self
    }
    pub fn translate_local(&mut self, offset: Vector3) -> &mut Self {
        let _ = self.get_transform().translated_local(offset);
        self
    }
}

impl Default for GdTransform3D {
    fn default() -> Self {
        Self {
            gd_transform: Transform3D::new(Basis::IDENTITY, Vector3::ZERO),
        }
    }
}
