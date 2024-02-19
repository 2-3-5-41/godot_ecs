use godot::prelude::{Gd, GodotClass, InstanceId};

#[derive(Clone, Copy, Debug)]
pub struct GdInstance(InstanceId);

impl GdInstance {
    pub fn new<G: GodotClass>(obj: Gd<G>) -> Self {
        Self(obj.instance_id())
    }
    pub fn try_bind<G: GodotClass>(&self) -> Option<Gd<G>> {
        match Gd::try_from_instance_id(self.0) {
            Ok(image) => Some(image),
            // API changed upstream. For now ignore the error.
            Err(_) => None,
        }
    }
    pub fn bind<G: GodotClass>(&self) -> Gd<G> {
        self.try_bind().unwrap()
    }
}
