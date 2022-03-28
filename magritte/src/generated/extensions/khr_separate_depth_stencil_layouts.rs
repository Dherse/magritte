//![VK_KHR_separate_depth_stencil_layouts](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_separate_depth_stencil_layouts.html) - device extension
//!# Description
//!This extension allows image memory barriers for depth/stencil images to have
//!just one of the `VK_IMAGE_ASPECT_DEPTH_BIT` or
//!`VK_IMAGE_ASPECT_STENCIL_BIT` aspect bits set, rather than require both.
//!This allows their layouts to be set independently.
//!To support depth/stencil images with different layouts for the depth and
//!stencil aspects, the depth/stencil attachment interface has been updated to
//!support a separate layout for stencil.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_create_renderpass2`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_separate_depth_stencil_layouts]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_separate_depth_stencil_layouts extension>>)
//!# New structures
//! - Extending [`AttachmentDescription2`]:  - [`AttachmentDescriptionStencilLayoutKHR`]
//! - Extending [`AttachmentReference2`]:  - [`AttachmentReferenceStencilLayoutKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR`]
//!# New constants
//! - [`KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME`]
//! - [`KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION`]
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR`  -
//!   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR`  -
//!   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR`  -
//!   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR`
//!   - `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2019-06-25 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2019-06-25
//! * - Promoted to Vulkan 1.2 Core
//! * - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA  - Jesse Barker, Unity  - Tobias Hector, AMD
//!# Related
//! - [`AttachmentDescriptionStencilLayoutKHR`]
//! - [`AttachmentReferenceStencilLayoutKHR`]
//! - [`PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR`]
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
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_separate_depth_stencil_layouts");
