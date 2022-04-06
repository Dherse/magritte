//! # Magritte
//! Magritte will be a single-backend, asynchronous graphics API for rust.
//! Inspired by wgpu-rs but designed for native Desktop use instead of
//! compatibility with `WebGPU`. The final API will be designed to enabled
//! high performance graphics and compute applications with the latest features.
#![feature(
    const_trait_impl,
    const_mut_refs,
    const_fn_fn_ptr_basics,
    generic_associated_types,
    arbitrary_self_types,
    try_trait_v2
)]
#![warn(clippy::pedantic, clippy::cargo)]
// #![deny(missing_docs)]

pub mod cstr;
pub mod generated;
pub mod version;
pub mod video;

pub mod entry;
pub mod handles;
#[cfg(feature = "libloading")]
pub mod loading;
pub mod results;
#[cfg(feature = "window")]
pub mod window;
pub mod helpers;

use generated::vulkan1_0::VulkanResultCodes;
pub use generated::{extensions::Extensions, *};
pub use handles::{AsRaw, Handle, Unique};
pub use results::VulkanResult;
pub use version::*;

#[cfg(feature = "smallvec")]
pub type SmallVec<T> = smallvec::SmallVec<[T; 8]>;

#[cfg(not(feature = "smallvec"))]
pub type SmallVec<T> = Vec<T>;

impl std::error::Error for VulkanResultCodes {}

impl std::fmt::Display for VulkanResultCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <VulkanResultCodes as std::fmt::Debug>::fmt(self, f)
    }
}
