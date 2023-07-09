#![feature(
    const_trait_impl,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    cfg_sanitize,
    specialization
)]

pub mod context;

#[cfg(feature = "native")]
mod entry;
mod version;

#[cfg(feature = "native")]
pub mod conv;

mod high_level {
    #[path = "../../generated/high_level/vulkan1_0.rs"]
    pub mod vulkan1_0;

    #[path = "../../generated/high_level/vulkan1_1.rs"]
    pub mod vulkan1_1;
}

#[cfg(feature = "native")]
pub mod native {
    #[path = "../../generated/native/opaque.rs"]
    pub mod opaque;

    #[path = "../../generated/native/vulkan1_0.rs"]
    pub mod vulkan1_0;

    #[path = "../../generated/native/vulkan1_1.rs"]
    pub mod vulkan1_1;

    #[path = "../../generated/native/vulkan1_2.rs"]
    #[cfg(feature = "VULKAN_1_2")]
    pub mod vulkan1_2;

    #[path = "../../generated/native/vulkan1_3.rs"]
    #[cfg(feature = "VULKAN_1_3")]
    pub mod vulkan1_3;

    #[path = "../../generated/native/extensions.rs"]
    pub mod extensions;

    #[path = "../../generated/native/api.rs"]
    pub mod api;
}

pub mod common {
    #[path = "../../generated/common/vulkan1_0.rs"]
    pub mod vulkan1_0;

    #[path = "../../generated/common/vulkan1_1.rs"]
    pub mod vulkan1_1;

    #[path = "../../generated/common/vulkan1_2.rs"]
    #[cfg(feature = "VULKAN_1_2")]
    pub mod vulkan1_2;

    #[path = "../../generated/common/vulkan1_3.rs"]
    #[cfg(feature = "VULKAN_1_3")]
    pub mod vulkan1_3;

    #[path = "../../generated/common/extensions.rs"]
    pub mod extensions;
}

pub use high_level::*;

#[cfg(feature = "native")]
pub use entry::Entry;

pub use smallvec::{smallvec, SmallVec};
pub use version::Version;
