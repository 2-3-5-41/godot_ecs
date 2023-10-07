pub(crate) mod macros {
    /// Quickly generates a default [`ResourceId`] type.
    ///
    /// ## Example
    /// ```rs
    /// // Creates a `Camera` struct with default implementations that generates the [`Rid`] on the [`RenderingServer`]
    /// resource_object!(Camera, camera_create, RenderingServer);
    ///
    /// // Creates an `Area` struct with default implementations that generates the [`Rid`] on the [`PhysicsServer2D`]
    /// resource_object!(Area, area_create, PhysicsServer2D);
    /// ```
    macro_rules! resource_object {
        (
            $resource_name: ident,
            $create: ident,
            $server: ident
        ) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $resource_name(Rid);

            impl ResourceId for $resource_name {
                fn get_rid(&self) -> Rid {
                    self.0
                }
                fn free_rid(&self) {
                    $server::singleton().free_rid(self.get_rid())
                }
            }

            impl $resource_name {
                pub fn create() -> Self {
                    Self($server::singleton().$create())
                }
                pub fn from_rid(rid: Rid) -> Self {
                    Self(rid)
                }
            }
        };
    }

    pub(crate) use resource_object;
}
