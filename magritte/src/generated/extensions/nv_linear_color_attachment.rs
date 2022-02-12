//![VK_NV_linear_color_attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_linear_color_attachment.html) - device extension
//!# Description
//!This extension expands support for using `VK_IMAGE_TILING_LINEAR` images
//!as color attachments when all the color attachments in the render pass
//!instance have `VK_IMAGE_TILING_LINEAR` tiling.
//!This extension adds a new flag bit
//!`VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV` that extends the
//!existing [`FormatFeatureFlagBits2KHR`] bits.
//!This flag **can** be set for renderable color formats in the
//![`FormatProperties3KHR::linear_tiling_features`] format properties
//!structure member.
//!Formats with the `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
//!flag **may** be used as color attachments as long as all the color attachments
//!in the render pass instance have `VK_IMAGE_TILING_LINEAR` tiling, and
//!the formats their images views are created with have
//![`FormatProperties3KHR::linear_tiling_features`] which include
//!`VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`.
//!This extension supports both dynamic rendering and traditional render
//!passes.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - sourav parmar [souravpNV](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_linear_color_attachment]
//!   @souravpNV%0A<<Here describe the issue or question you have about the
//!   VK_NV_linear_color_attachment extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceLinearColorAttachmentFeaturesNV`]
//!# New constants
//! - [`NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME`]
//! - [`NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
//!
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:
//! - `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
//!# Version History
//! - Revision 1, 2021-11-29 (sourav parmar)
//! - Initial draft
//!# Other info
//! * 2021-12-02
//!*
//! - This extension requires [`VK_KHR_format_feature_flags2`]
//!
//!*
//! - Pat Brown, NVIDIA
//! - Piers Daniell, NVIDIA
//! - Sourav Parmar, NVIDIA
//!# Related
//! - [`PhysicalDeviceLinearColorAttachmentFeaturesNV`]
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
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION")]
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME")]
pub const NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_linear_color_attachment");
