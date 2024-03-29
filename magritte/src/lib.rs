//! # Magritte
//! Magritte will be a single-backend, asynchronous graphics API for rust.
//! Inspired by wgpu-rs but designed for native Desktop use instead of
//! compatibility with `WebGPU`. The final API will be designed to enabled
//! high performance graphics and compute applications with the latest features.
#![feature(
    const_trait_impl,
    const_mut_refs,
    arbitrary_self_types,
    try_trait_v2,
    cfg_sanitize,
    const_cstr_methods
)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(unreachable_code)]
// #![deny(missing_docs)]

pub mod cstr;
pub mod generated;
pub mod version;
pub mod video;

pub mod chaining;
pub mod commands;
pub mod descriptors;
pub mod entry;
#[doc(hidden)]
pub mod fence;
pub mod handles;
pub mod helpers;
#[cfg(feature = "libloading")]
pub mod loading;
pub mod memory;
pub mod results;
pub mod size;
pub mod spv;
pub mod utils;
#[cfg(feature = "validation")]
pub mod validation;
#[cfg(feature = "window")]
pub mod window;

use std::ffi::CStr;

pub use chaining::Chain;
use generated::vulkan1_0::VulkanResultCodes;
pub use generated::{
    extensions::{DeviceExtensions, InstanceExtensions},
    *,
};
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
        f.write_str(match *self {
            Self::SUCCESS => "success",
            Self::NOT_READY => "a fence query has not yet finished",
            Self::TIMEOUT => "a wait operation has timed out",
            Self::EVENT_SET => "an event is signaled",
            Self::EVENT_RESET => "an event is unsignaled",
            Self::INCOMPLETE => "a return array was too small to fit all of the data",
            Self::ERROR_OUT_OF_HOST_MEMORY => "a host memory allocation has failed",
            Self::ERROR_OUT_OF_DEVICE_MEMORY => "a device memory allocation has failed",
            Self::ERROR_INITIALIZATION_FAILED => "initialization of an object could not be completed for implementation-specific reasons",
            Self::ERROR_DEVICE_LOST => "the logical or physical device has been lost",
            Self::ERROR_MEMORY_MAP_FAILED => "mapping of a memory object has failed",
            Self::ERROR_LAYER_NOT_PRESENT => "a requested layer is not present or could not be loaded",
            Self::ERROR_EXTENSION_NOT_PRESENT => "a requested extension is not supported",
            Self::ERROR_FEATURE_NOT_PRESENT => "a requested feature is not supported",
            Self::ERROR_INCOMPATIBLE_DRIVER => "the requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons",
            Self::ERROR_TOO_MANY_OBJECTS => "too many objects of the type have already been created",
            Self::ERROR_FORMAT_NOT_SUPPORTED => "a requested format is not supported on this device",
            Self::ERROR_FRAGMENTED_POOL => "a pool allocation has failed due to fragmentation of the pool's memory",
            Self::ERROR_UNKNOWN => "an unknown error has occurred; either the application has provided invalid input, or an implementation failure has occurred",
            Self::ERROR_OUT_OF_POOL_MEMORY => "ERROR_OUT_OF_POOL_MEMORY",
            Self::ERROR_INVALID_EXTERNAL_HANDLE => "an external handle is not a valid handle of the specified type",
            Self::ERROR_FRAGMENTATION => "a descriptor pool creation has failed due to fragmentation",
            Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => 
                "a buffer creation or memory allocation failed because the requested address is not available"
            ,
            Self::PIPELINE_COMPILE_REQUIRED => "a requested pipeline creation would have required compilation, but the application requested compilation to not be performed",
            #[cfg(feature = "VK_KHR_surface")]
            Self::ERROR_SURFACE_LOST_KHR => "a surface is no longer available",
            #[cfg(feature = "VK_KHR_surface")]
            Self::ERROR_NATIVE_WINDOW_IN_USE_KHR => "The requested window is already in use by Vulkan or another API in a manner which prevents it from being used again",
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SUBOPTIMAL_KHR => "a swapchain no longer matches the surface properties exactly, but can still be used to present to the surface successfully",
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::ERROR_OUT_OF_DATE_KHR => "a surface has changed in such a way that it is no longer compatible with the swapchain, and further presentation requests using the swapchain will fail. Applications must query the new surface properties and recreate their swapchain if they wish to continue presenting to the surface",
            #[cfg(feature = "VK_KHR_display_swapchain")]
            Self::ERROR_INCOMPATIBLE_DISPLAY_KHR => "the display used by a swapchain does not use the same presentable image layout, or is incompatible in a way that prevents sharing an image",
            #[cfg(feature = "VK_EXT_debug_report")]
            Self::ERROR_VALIDATION_FAILED_EXT => "ERROR_VALIDATION_FAILED_EXT",
            #[cfg(feature = "VK_NV_glsl_shader")]
            Self::ERROR_INVALID_SHADER_NV => "One or more shaders failed to compile or link",
            #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
            Self::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT =>
                "ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT",
            #[cfg(feature = "VK_KHR_global_priority")]
            Self::ERROR_NOT_PERMITTED_KHR => "ERROR_NOT_PERMITTED_KHR",
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            Self::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT =>
                "an operation on a swapchain created with `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` failed as it did not have exlusive full-screen access",
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::THREAD_IDLE_KHR => "a deferred operation is not complete but there is currently no work for this thread to do at the time of this call",
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::THREAD_DONE_KHR => "a deferred operation is not complete but there is no work remaining to assign to additional threads",
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::OPERATION_DEFERRED_KHR => "a deferred operation was requested and at least some of the work was deferred",
            #[cfg(feature = "VK_KHR_deferred_host_operations")]
            Self::OPERATION_NOT_DEFERRED_KHR => "a deferred operation was requested and no operations were deferred",
            _ => "invalid",
        })
    }
}

pub trait AsCStr {
    fn as_cstr(&self) -> &CStr;
}

impl AsCStr for &[i8] {
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}

impl<const N: usize> AsCStr for &[i8; N] {
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}

impl<const N: usize> AsCStr for [i8; N] {
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}
