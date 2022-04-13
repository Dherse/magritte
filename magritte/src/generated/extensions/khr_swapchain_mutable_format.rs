//![VK_KHR_swapchain_mutable_format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain_mutable_format.html) - device extension
//!# Description
//!This extension allows processing of swapchain images as different formats to
//!that used by the window system, which is particularly useful for switching
//!between sRGB and linear RGB formats.It adds a new swapchain creation flag that enables creating
//! image views from
//!presentable images with a different format than the one used to create the
//!swapchain.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//! - Requires `[`khr_maintenance2`]`
//! - Requires `[`khr_image_format_list`]`
//!# Contacts
//! - Daniel Rakos [drakos-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_swapchain_mutable_format]
//!   @drakos-arm%0A<<Here describe the issue or question you have about the
//!   VK_KHR_swapchain_mutable_format extension>>)
//!# New constants
//! - [`KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME`]
//! - [`KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION`]
//! - Extending [`SwapchainCreateFlagBitsKHR`]:  - `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR`
//!# Known issues & F.A.Q
//!1) Are there any new capabilities needed? **RESOLVED** : No.
//!It is expected that all implementations exposing this extension support
//!swapchain image format mutability.2) Do we need a separate
//! `VK_SWAPCHAIN_CREATE_EXTENDED_USAGE_BIT_KHR`? **RESOLVED** : No.
//!This extension requires [`VK_KHR_maintenance2`] and presentable images of
//!swapchains created with `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR` are
//!created internally in a way equivalent to specifying both
//!`VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` and
//!`VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR`.3) Do we need a separate structure to allow specifying
//! an image format list
//!for swapchains? **RESOLVED** : No.
//!We simply use the same [`ImageFormatListCreateInfoKHR`] structure
//!introduced by [`VK_KHR_image_format_list`].
//!The structure is required to be included in the `pNext` chain of
//![`SwapchainCreateInfoKHR`] for swapchains created with
//!`VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR`.
//!# Version History
//! - Revision 1, 2018-03-28 (Daniel Rakos)  - Internal revisions.
//!# Other info
//! * 2018-03-28
//! * No known IP claims.
//! * - Jason Ekstrand, Intel  - Jan-Harald Fredriksen, ARM  - Jesse Hall, Google  - Daniel Rakos,
//!   AMD  - Ray Smith, ARM
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_swapchain_mutable_format");
