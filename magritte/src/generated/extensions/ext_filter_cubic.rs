//![VK_EXT_filter_cubic](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_filter_cubic.html) - device extension
//!# Description
//![`VK_EXT_filter_cubic`] extends [`VK_IMG_filter_cubic`].It documents cubic filtering of other
//! image view types.
//!It adds new structures that **can** be added to the `pNext` chain of
//![`PhysicalDeviceImageFormatInfo2`] and [`ImageFormatProperties2`]
//!that **can** be used to determine which image types and which image view types
//!support cubic filtering.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Bill Licea-Kane [wwlk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_filter_cubic]
//!   @wwlk%0A<<Here describe the issue or question you have about the VK_EXT_filter_cubic
//!   extension>>)
//!# New structures
//! - Extending [`ImageFormatProperties2`]:
//! - [`FilterCubicImageViewImageFormatPropertiesEXT`]
//!
//! - Extending [`PhysicalDeviceImageFormatInfo2`]:
//! - [`PhysicalDeviceImageViewImageFormatInfoEXT`]
//!# New constants
//! - [`EXT_FILTER_CUBIC_EXTENSION_NAME`]
//! - [`EXT_FILTER_CUBIC_SPEC_VERSION`]
//! - Extending [`Filter`]:
//! - `VK_FILTER_CUBIC_EXT`
//!
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
//!# Version History
//! - Revision 3, 2019-12-13 (wwlk)
//! - Delete requirement to cubic filter the formats USCALED_PACKED32,
//!SSCALED_PACKED32, UINT_PACK32, and SINT_PACK32 (cut/paste error)
//!
//! - Revision 2, 2019-06-05 (wwlk)
//! - Clarify 1D optional
//!
//! - Revision 1, 2019-01-24 (wwlk)
//! - Initial version
//!# Other info
//! * 2019-12-13
//!*
//! - Bill Licea-Kane, Qualcomm Technologies, Inc.
//! - Andrew Garrard, Samsung
//! - Daniel Koch, NVIDIA
//! - Donald Scorgie, Imagination Technologies
//! - Graeme Leese, Broadcom
//! - Jan-Herald Fredericksen, ARM
//! - Jeff Leger, Qualcomm Technologies, Inc.
//! - Tobias Hector, AMD
//! - Tom Olson, ARM
//! - Stuart Smith, Imagination Technologies
//!# Related
//! - [`FilterCubicImageViewImageFormatPropertiesEXT`]
//! - [`PhysicalDeviceImageViewImageFormatInfoEXT`]
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
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_filter_cubic");
