use godot::{
    engine::RenderingServer,
    prelude::{Rid, Transform2D, Transform3D},
};

use crate::resources::{traits::ResourceId, utils::macros::resource_object};

resource_object!(Skeleton, skeleton_create, RenderingServer);

impl Skeleton {
    pub fn allocate_data(&self, bones: i32, is_2d_skeleton: Option<bool>) -> &Self {
        if let Some(is_2d_skeleton) = is_2d_skeleton {
            RenderingServer::singleton()
                .skeleton_allocate_data_ex(self.get_rid(), bones)
                .is_2d_skeleton(is_2d_skeleton)
                .done();
        } else {
            RenderingServer::singleton().skeleton_allocate_data(self.get_rid(), bones);
        }
        self
    }
    pub fn bone_get_transform(&self, bone: i32) -> Transform3D {
        RenderingServer::singleton().skeleton_bone_get_transform(self.get_rid(), bone)
    }
    pub fn bone_get_transform_2d(&self, bone: i32) -> Transform2D {
        RenderingServer::singleton().skeleton_bone_get_transform_2d(self.get_rid(), bone)
    }
    pub fn bone_set_transform(&self, bone: i32, transform: Transform3D) -> &Self {
        RenderingServer::singleton().skeleton_bone_set_transform(self.get_rid(), bone, transform);
        self
    }
    pub fn bone_set_transform_2d(&self, bone: i32, transform: Transform2D) -> &Self {
        RenderingServer::singleton().skeleton_bone_set_transform_2d(
            self.get_rid(),
            bone,
            transform,
        );
        self
    }
    pub fn get_bone_count(&self) -> i32 {
        RenderingServer::singleton().skeleton_get_bone_count(self.get_rid())
    }
    pub fn set_base_transform_2d(&self, base_transform: Transform2D) -> &Self {
        RenderingServer::singleton().skeleton_set_base_transform_2d(self.get_rid(), base_transform);
        self
    }
}
