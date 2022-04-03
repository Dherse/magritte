//! # Magritte
//! Magritte will be a single-backend, asynchronous graphics API for rust.
//! Inspired by wgpu-rs but designed for native Desktop use instead of
//! compatibility with `WebGPU`. The final API will be designed to enabled
//! high performance graphics and compute applications with the latest features.
#![feature(const_trait_impl, const_mut_refs, const_fn_fn_ptr_basics)]
#![warn(clippy::pedantic, clippy::cargo)]
// #![deny(missing_docs)]

pub mod cstr;
pub mod generated;
pub mod version;
pub mod video;

pub mod entry;
#[cfg(feature = "libloading")]
pub mod loading;

pub use generated::{extensions::Extensions, *};
pub use version::*;
