use mcrs_minecraft_resource::{DEFAULT_NAMESPACE, ResourceLocation};

macro_rules! builtin_key {
    ($($name:ident => $path:literal),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum BuiltinKey {
            $($name,)*
        }

        const ALL: &[BuiltinKey] = &[$(BuiltinKey::$name,)*];

        impl BuiltinKey {
            #[must_use]
            pub const fn path(self) -> &'static str {
                match self {
                    $(Self::$name => $path,)*
                }
            }
        }
    };
}

crate::for_each_builtin!(builtin_key);

impl BuiltinKey {
    #[doc(hidden)]
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub fn location(self) -> Option<ResourceLocation<'static>> {
        ResourceLocation::new(DEFAULT_NAMESPACE, self.path()).ok()
    }

    #[must_use]
    pub fn from_location(location: ResourceLocation<'_>) -> Option<Self> {
        if location.namespace() != DEFAULT_NAMESPACE {
            return None;
        }
        ALL.iter()
            .copied()
            .find(|key| key.path() == location.path())
    }
}
