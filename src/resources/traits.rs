use godot::prelude::Rid;

pub trait ResourceId: Clone + Copy {
    fn get_rid(&self) -> Rid {
        unimplemented!()
    }
    fn free_rid(&self) {
        unimplemented!()
    }
}

pub trait RenderableObj: ResourceId {
    fn create() -> Self {
        unimplemented!()
    }
    #[allow(unused_variables)]
    fn from_rid(rid: Rid) -> Self {
        unimplemented!()
    }
}
