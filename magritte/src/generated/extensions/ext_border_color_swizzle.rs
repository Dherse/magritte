//![VK_EXT_border_color_swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_border_color_swizzle.html) - device extension
//!# Description
//!After the publication of VK_EXT_custom_border_color, it was discovered that
//!some implementations had undefined behavior when combining a sampler that
//!uses a custom border color with image views whose component mapping is not
//!the identity mapping.Since VK_EXT_custom_border_color has already shipped, this new extension
//!VK_EXT_border_color_swizzle was created to define the interaction between
//!custom border colors and non-identity image view swizzles, and provide a
//!work-around for implementations that must pre-swizzle the sampler border
//!color to match the image view component mapping it is combined with.This extension also defines
//! the behavior between samplers with an opaque
//!black border color and image views with a non-identity component swizzle,
//!which was previously left undefined.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_custom_border_color`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_border_color_swizzle]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_border_color_swizzle extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]
//! - Extending [`SamplerCreateInfo`]:
//! - [`SamplerBorderColorComponentMappingCreateInfoEXT`]
//!# New constants
//! - [`EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME`]
//! - [`EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2021-10-12 (Piers Daniell)
//! - Internal revisions.
//!# Other info
//! * 2021-10-12
//! * No known IP claims.
//!*
//! - Graeme Leese, Broadcom
//! - Jan-Harald Fredriksen, Arm
//! - Ricardo Garcia, Igalia
//! - Shahbaz Youssefi, Google
//! - Stu Smith, AMD
//!# Related
//! - [`PhysicalDeviceBorderColorSwizzleFeaturesEXT`]
//! - [`SamplerBorderColorComponentMappingCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_border_color_swizzle");
