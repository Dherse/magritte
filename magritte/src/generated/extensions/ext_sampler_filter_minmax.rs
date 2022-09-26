//![VK_EXT_sampler_filter_minmax](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sampler_filter_minmax.html) - device extension
//!# Description
//!In unextended Vulkan, minification and magnification filters such as LINEAR
//!allow sampled image lookups to return a filtered texel value produced by
//!computing a weighted average of a collection of texels in the neighborhood
//!of the texture coordinate provided.This extension provides a new sampler parameter which allows
//! applications to
//!produce a filtered texel value by computing a component-wise minimum (MIN)
//!or maximum (MAX) of the texels that would normally be averaged.
//!The reduction mode is orthogonal to the minification and magnification
//!filter parameters.
//!The filter parameters are used to identify the set of texels used to produce
//!a final filtered value; the reduction mode identifies how these texels are
//!combined.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_sampler_filter_minmax]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_sampler_filter_minmax extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceSamplerFilterMinmaxPropertiesEXT`]
//! - Extending [`SamplerCreateInfo`]:  - [`SamplerReductionModeCreateInfoEXT`]
//!# New enums
//! - [`SamplerReductionModeEXT`]
//!# New constants
//! - [`EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME`]
//! - [`EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION`]
//! - Extending [`FormatFeatureFlagBits`]:  -
//!   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT`
//! - Extending [`SamplerReductionMode`]:  - `VK_SAMPLER_REDUCTION_MODE_MAX_EXT`  -
//!   `VK_SAMPLER_REDUCTION_MODE_MIN_EXT`  - `VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT`
//!# Version history
//! - Revision 2, 2017-05-19 (Piers Daniell)  - Renamed to EXT
//! - Revision 1, 2017-03-25 (Jeff Bolz)  - Internal revisions
//!# Other information
//! * 2017-05-19
//! * - Promoted to Vulkan 1.2 Core
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA  - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceSamplerFilterMinmaxPropertiesEXT`]
//! - [`SamplerReductionModeCreateInfoEXT`]
//! - [`SamplerReductionModeEXT`]
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
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION")]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME")]
pub const EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_sampler_filter_minmax");
