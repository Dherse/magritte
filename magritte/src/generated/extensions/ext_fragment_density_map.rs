//![VK_EXT_fragment_density_map](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map.html) - device extension
//!# Description
//!This extension allows an application to specify areas of the render target
//!where the fragment shader may be invoked fewer times.
//!These fragments are broadcasted out to multiple pixels to cover the render
//!target.The primary use of this extension is to reduce workloads in areas where
//!lower quality may not be perceived such as the distorted edges of a lens or
//!the periphery of a user’s gaze.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Matthew Netsch [mnetsch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_fragment_density_map]
//!   @mnetsch%0A<<Here describe the issue or question you have about the
//!   VK_EXT_fragment_density_map extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
//!
//! - Extending [`RenderPassCreateInfo`], [`RenderPassCreateInfo2`]:
//! - [`RenderPassFragmentDensityMapCreateInfoEXT`]
//!# New constants
//! - [`EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME`]
//! - [`EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`
//!
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//!
//! - Extending [`ImageCreateFlagBits`]:
//! - `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
//!
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
//!
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//!
//! - Extending [`ImageViewCreateFlagBits`]:
//! - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`
//!
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
//!
//! - Extending [`SamplerCreateFlagBits`]:
//! - `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`
//! - `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
//! - `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
//!
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:
//! - `VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-09-25 (Matthew Netsch)
//! - Initial version
//!
//! - Revision 2, 2021-09-30 (Jon Leech)
//! - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`
//!# Other info
//! * 2021-09-30
//!*
//! - This extension requires
//![`SPV_EXT_fragment_invocation_density`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_fragment_invocation_density.html)
//! - This extension provides API support for
//![`GL_EXT_fragment_invocation_density`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_fragment_invocation_density.txt)
//!
//!*
//! - Matthew Netsch, Qualcomm Technologies, Inc.
//! - Robert VanReenen, Qualcomm Technologies, Inc.
//! - Jonathan Wicks, Qualcomm Technologies, Inc.
//! - Tate Hornbeck, Qualcomm Technologies, Inc.
//! - Sam Holmes, Qualcomm Technologies, Inc.
//! - Jeff Leger, Qualcomm Technologies, Inc.
//! - Jan-Harald Fredriksen, ARM
//! - Jeff Bolz, NVIDIA
//! - Pat Brown, NVIDIA
//! - Daniel Rakos, AMD
//! - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
//! - [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
//! - [`RenderPassFragmentDensityMapCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map");
