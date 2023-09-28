use godot::prelude::Rid;

pub trait ResourceId: Clone + Copy {
    fn get_rid(&self) -> Rid {
        unimplemented!()
    }
    fn free_rid(&self) {
        unimplemented!()
    }
}
