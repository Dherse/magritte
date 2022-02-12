//![VK_KHR_maintenance1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance1.html) - device extension
//!# Description
//![`VK_KHR_maintenance1`] adds a collection of minor features that were
//!intentionally left out or overlooked from the original Vulkan 1.0 release.The new features are
//! as follows:
//! - Allow 2D and 2D array image views to be created from 3D images, which
//!can then be used as color framebuffer attachments.
//!This allows applications to render to slices of a 3D image.
//! - Support [`CmdCopyImage`] between 2D array layers and 3D slices.
//!This extension allows copying from layers of a 2D array image to slices
//!of a 3D image and vice versa.
//! - Allow negative height to be specified in the
//![`Viewport::height`] field to perform y-inversion of the
//!clip-space to framebuffer-space transform.
//!This allows apps to avoid having to use `gl_Position.y = -gl_Position.y`
//!in shaders also targeting other APIs.
//! - Allow implementations to express support for doing just transfers and
//!clears of image formats that they otherwise support no other format
//!features for.
//!This is done by adding new format feature flags
//!`VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR` and
//!`VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR`.
//! - Support [`CmdFillBuffer`] on transfer-only queues.
//!Previously [`CmdFillBuffer`] was defined to only work on command
//!buffers allocated from command pools which support graphics or compute
//!queues.
//!It is now allowed on queues that just support transfer operations.
//! - Fix the inconsistency of how error conditions are returned between the
//![`CreateGraphicsPipelines`] and [`CreateComputePipelines`]
//!functions and the [`AllocateDescriptorSets`] and
//![`AllocateCommandBuffers`] functions.
//! - Add new `VK_ERROR_OUT_OF_POOL_MEMORY_KHR` error so implementations
//!can give a more precise reason for [`AllocateDescriptorSets`]
//!failures.
//! - Add a new command [`TrimCommandPoolKHR`] which gives the
//!implementation an opportunity to release any unused command pool memory
//!back to the system.
//!# Revision
//!2
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_maintenance1]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_KHR_maintenance1
//!   extension>>)
//!# New functions & commands
//! - [`TrimCommandPoolKHR`]
//!# New bitmasks
//! - [`CommandPoolTrimFlagsKHR`]
//!# New constants
//! - [`KHR_MAINTENANCE1_EXTENSION_NAME`]
//! - [`KHR_MAINTENANCE1_SPEC_VERSION`]
//! - [`KHR_MAINTENANCE_1_EXTENSION_NAME`]
//! - [`KHR_MAINTENANCE_1_SPEC_VERSION`]
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR`
//! - `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR`
//! - Extending [`ImageCreateFlagBits`]:
//! - `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR`
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_OUT_OF_POOL_MEMORY_KHR`
//!# Known issues & F.A.Q
//!0. Are viewports with zero height allowed?**RESOLVED**: Yes, although they have low utility.
//!# Version History
//! - Revision 1, 2016-10-26 (Piers Daniell)
//! - Internal revisions
//! - Revision 2, 2018-03-13 (Jon Leech)
//! - Add issue for zero-height viewports
//!# Other info
//! * 2018-03-13
//!*
//! - Promoted to Vulkan 1.1 Core
//!*
//! - Dan Ginsburg, Valve
//! - Daniel Koch, NVIDIA
//! - Daniel Rakos, AMD
//! - Jan-Harald Fredriksen, ARM
//! - Jason Ekstrand, Intel
//! - Jeff Bolz, NVIDIA
//! - Jesse Hall, Google
//! - John Kessenich, Google
//! - Michael Worcester, Imagination Technologies
//! - Neil Henning, Codeplay Software Ltd.
//! - Piers Daniell, NVIDIA
//! - Slawomir Grajewski, Intel
//! - Tobias Hector, Imagination Technologies
//! - Tom Olson, ARM
//!# Related
//! - [`CommandPoolTrimFlagsKHR`]
//! - [`TrimCommandPoolKHR`]
//!
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
#[doc(alias = "VK_KHR_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_1_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_maintenance1");
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MAINTENANCE1_SPEC_VERSION")]
pub const KHR_MAINTENANCE1_SPEC_VERSION: u32 = KHR_MAINTENANCE_1_SPEC_VERSION;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MAINTENANCE1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE1_EXTENSION_NAME: &'static CStr = KHR_MAINTENANCE_1_EXTENSION_NAME;
