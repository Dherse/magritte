#![feature(const_trait_impl, const_mut_refs)]

#[path = "../generated/opaque.rs"]
pub mod opaque;

#[path = "../generated/vulkan1_0.rs"]
pub mod vulkan1_0;

#[path = "../generated/vulkan1_1.rs"]
pub mod vulkan1_1;

#[cfg(feature = "VULKAN_1_2")]
#[path = "../generated/vulkan1_2.rs"]
pub mod vulkan1_2;

#[cfg(feature = "VULKAN_1_3")]
#[path = "../generated/vulkan1_3.rs"]
pub mod vulkan1_3;

#[path = "../generated/extensions.rs"]
pub mod extensions;

pub use magritte_common::*;