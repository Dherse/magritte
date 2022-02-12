//![VK_QCOM_fragment_density_map_offset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_fragment_density_map_offset.html) - device extension
//!# Description
//!This extension allows an application to specify offsets to a fragment
//!density map attachment, changing the framebuffer location where density
//!values are applied to without having to regenerate the fragment density map.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_EXT_fragment_density_map`]`
//!# Contacts
//! - Matthew Netsch [mnetsch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_fragment_density_map_offset]
//!   @mnetsch%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_fragment_density_map_offset extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]
//! - Extending [`SubpassEndInfo`]:
//! - [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
//!# New constants
//! - [`QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME`]
//! - [`QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION`]
//! - Extending [`ImageCreateFlagBits`]:
//! - `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`
//! - `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
//!# Version History
//! - Revision 1, 2021-09-03 (Matthew Netsch)
//! - Initial version
//!# Other info
//! * 2021-09-03
//!*
//! - Matthew Netsch, Qualcomm Technologies, Inc.
//! - Jonathan Wicks, Qualcomm Technologies, Inc.
//! - Jonathan Tinkham, Qualcomm Technologies, Inc.
//! - Jeff Leger, Qualcomm Technologies, Inc.
//!# Related
//! - [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]
//! - [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]
//! - [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
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
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_QCOM_fragment_density_map_offset");
