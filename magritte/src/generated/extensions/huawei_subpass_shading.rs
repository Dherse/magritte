//![VK_HUAWEI_subpass_shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_subpass_shading.html) - device extension
//!# Description
//!This extension allows applications to execute a subpass shading pipeline in
//!a subpass of a render pass in order to save memory bandwidth for algorithms
//!like tile-based deferred rendering and forward plus.
//!A subpass shading pipeline is a pipeline with the compute pipeline ability,
//!allowed to read values from input attachments, and only allowed to be
//!dispatched inside a stand-alone subpass.
//!Its work dimension is defined by the render pass’s render area size.
//!Its workgroup size (width, height) shall be a power-of-two number in width
//!or height, with minimum value from 8, and maximum value shall be decided
//!from the render pass attachments and sample counts but depends on
//!implementation.The `GlobalInvocationId.xy` of a subpass shading pipeline is equal to the
//!`FragCoord.xy` of a graphic pipeline in the same render pass subtracted
//!the [`Rect2D`] of the
//![`RenderPassBeginInfo::render_area`].
//!`GlobalInvocationId.z` is mapped to the Layer if
//!`[`VK_EXT_shader_viewport_index_layer`]` is supported.
//!The `GlobalInvocationId.xy` is equal to the index of the local workgroup
//!multiplied by the size of the local workgroup plus the
//!`LocalInvocationId` and the [`Rect2D`] of the
//![`RenderPassBeginInfo::render_area`].This extension allows a subpass’s pipeline bind point to be
//!`VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_create_renderpass2`]`
//! - Requires `[`VK_KHR_synchronization2`]`
//!# Contacts
//! - Hueilong Wang [wyvernathuawei](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_HUAWEI_subpass_shading]
//!   @wyvernathuawei%0A<<Here describe the issue or question you have about the
//!   VK_HUAWEI_subpass_shading extension>>)
//!# New functions & commands
//! - [`CmdSubpassShadingHUAWEI`]
//! - [`GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`]
//!# New structures
//! - Extending [`ComputePipelineCreateInfo`]:
//! - [`SubpassShadingPipelineCreateInfoHUAWEI`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`]
//!# New constants
//! - [`HUAWEI_SUBPASS_SHADING_EXTENSION_NAME`]
//! - [`HUAWEI_SUBPASS_SHADING_SPEC_VERSION`]
//! - Extending [`PipelineBindPoint`]:
//! - `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
//!
//! - Extending [`PipelineStageFlagBits2`]:
//! - `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
//!
//! - Extending [`ShaderStageFlagBits`]:
//! - `VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
//! - `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
//!# Version History
//! - Revision 2, 2021-06-28 (Hueilong Wang)
//! - Change vkGetSubpassShadingMaxWorkgroupSizeHUAWEI to
//!vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI to resolve issue
//![`pub1564`](https://github.com/KhronosGroup/Vulkan-Docs/issues/1564)
//!
//! - Revision 1, 2020-12-15 (Hueilong Wang)
//! - Initial draft.
//!# Other info
//! * 2021-06-01
//!*
//! - This extension requires
//![`SPV_HUAWEI_subpass_shading`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/HUAWEI/SPV_HUAWEI_subpass_shading.html).
//! - This extension provides API support for
//![`GL_HUAWEI_subpass_shading`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/huawei/GLSL_HUAWEI_subpass_shading.txt).
//!
//!*
//! - Hueilong Wang
//!# Related
//! - [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]
//! - [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`]
//! - [`SubpassShadingPipelineCreateInfoHUAWEI`]
//! - [`CmdSubpassShadingHUAWEI`]
//! - [`GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`]
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
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_HUAWEI_subpass_shading");
