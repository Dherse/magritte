//![VK_HUAWEI_invocation_mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_invocation_mask.html) - device extension
//!# Description
//!The rays to trace may be sparse in some use cases.
//!For example, the scene only have a few regions to reflect.
//!Providing an invocation mask image to the ray tracing commands could
//!potentially give the hardware the hint to do certain optimization without
//!invoking an additional pass to compact the ray buffer.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_ray_tracing_pipeline`]`
//! - Requires `[`VK_KHR_synchronization2`]`
//!# Contacts
//! - Yunpeng Zhu [yunxingzhu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_HUAWEI_invocation_mask]
//!   @yunxingzhu%0A<<Here describe the issue or question you have about the
//!   VK_HUAWEI_invocation_mask extension>>)
//!# New functions & commands
//! - [`CmdBindInvocationMaskHUAWEI`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`]
//!# New constants
//! - [`HUAWEI_INVOCATION_MASK_EXTENSION_NAME`]
//! - [`HUAWEI_INVOCATION_MASK_SPEC_VERSION`]
//! - Extending [`AccessFlagBits2`]:
//! - `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI`
//! - Extending [`PipelineStageFlagBits2`]:
//! - `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI`
//!# Version History
//! - Revision 1, 2021-05-27 (Yunpeng Zhu)
//! - Initial draft.
//!# Other info
//! * 2021-05-27
//!*
//! - This extension requires `[`VK_KHR_ray_tracing_pipeline`]`, which
//!allow to bind an invocation mask image before the ray tracing command
//! - This extension requires `[`VK_KHR_synchronization2`]`, which allows
//!new pipeline stage for the invocation mask image
//!*
//! - Yunpeng Zhu, HuaWei
//!# Related
//! - [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`]
//! - [`CmdBindInvocationMaskHUAWEI`]
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
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION")]
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME")]
pub const HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_HUAWEI_invocation_mask");
