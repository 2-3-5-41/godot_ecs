pub(crate) mod macros {
    macro_rules! renderable_object {
        (
            $resource_name: ident,
            $create: ident
        ) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $resource_name(Rid);

            impl ResourceId for $resource_name {
                fn get_rid(&self) -> Rid {
                    self.0
                }
                fn free_rid(&self) {
                    RenderingServer::singleton().free_rid(self.get_rid())
                }
            }

            impl RenderableObj for $resource_name {
                fn create() -> Self {
                    Self(RenderingServer::singleton().$create())
                }
                fn from_rid(rid: Rid) -> Self {
                    Self(rid)
                }
            }
        };
    }

    pub(crate) use renderable_object;
}
