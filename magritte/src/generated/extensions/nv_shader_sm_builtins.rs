//![VK_NV_shader_sm_builtins](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_sm_builtins.html) - device extension
//!# Description
//!This extension provides the ability to determine device-specific properties
//!on NVIDIA GPUs.
//!It provides the number of streaming multiprocessors (SMs), the maximum
//!number of warps (subgroups) that can run on an SM, and shader builtins to
//!enable invocations to identify which SM and warp a shader invocation is
//!executing on.This extension enables support for the SPIR-V `ShaderSMBuiltinsNV`
//!capability.These properties and built-ins **should** typically only be used for debugging
//!purposes.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shader_sm_builtins]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_NV_shader_sm_builtins
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`]
//!# New constants
//! - [`NV_SHADER_SM_BUILTINS_EXTENSION_NAME`]
//! - [`NV_SHADER_SM_BUILTINS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!0. What should we call this extension?**RESOLVED**: `NV_shader_sm_builtins`.
//!Other options considered included:
//! - `NV_shader_smid` - but SMID is really easy to typo/confuse as SIMD.
//! - `NV_shader_sm_info` - but **Info** is typically reserved for input
//!structures
//!# Version History
//! - Revision 1, 2019-05-28 (Daniel Koch)
//! - Internal revisions
//!# Other info
//! * 2019-05-28
//!*
//! - This extension requires
//![`SPV_NV_shader_sm_builtins`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_sm_builtins.html).
//! - This extension provides API support for
//![`GL_NV_shader_sm_builtins`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shader_sm_builtins.txt)
//!
//!*
//! - Jeff Bolz, NVIDIA
//! - Eric Werness, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`]
//! - [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`]
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
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION")]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME")]
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_shader_sm_builtins");
