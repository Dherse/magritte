//![VK_EXT_fragment_density_map2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map2.html) - device extension
//!# Description
//!This extension adds additional features and properties to
//!`[`VK_EXT_fragment_density_map`]` in order to reduce fragment density map
//!host latency as well as improved queries for subsampled sampler
//!implementation-dependent behavior.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_fragment_density_map`]`
//!# Contacts
//! - Matthew Netsch [mnetsch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_fragment_density_map2]
//!   @mnetsch%0A<<Here describe the issue or question you have about the
//!   VK_EXT_fragment_density_map2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
//! - Extending [`RenderPassCreateInfo`], [`RenderPassCreateInfo2`]:
//! - [`RenderPassFragmentDensityMapCreateInfoEXT`]
//!# New constants
//! - [`EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME`]
//! - [`EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//! - Extending [`ImageCreateFlagBits`]:
//! - `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//! - Extending [`ImageViewCreateFlagBits`]:
//! - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
//! - Extending [`SamplerCreateFlagBits`]:
//! - `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`
//! - `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
//! - `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`If
//!   [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:
//! - `VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT`
//!# Version History
//! - Revision 1, 2020-06-16 (Matthew Netsch)
//! - Initial version
//!# Other info
//! * 2020-06-16
//!*
//! - Interacts with Vulkan 1.1
//!*
//! - Matthew Netsch, Qualcomm Technologies, Inc.
//! - Jonathan Tinkham, Qualcomm Technologies, Inc.
//! - Jonathan Wicks, Qualcomm Technologies, Inc.
//! - Jan-Harald Fredriksen, ARM
//!# Related
//! - [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`]
//! - [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`]
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
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map2");
