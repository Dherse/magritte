//![VK_EXT_shader_demote_to_helper_invocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_demote_to_helper_invocation.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_EXT_demote_to_helper_invocation`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_demote_to_helper_invocation.html)
//!SPIR-V extension.
//!That SPIR-V extension provides a new instruction
//!`OpDemoteToHelperInvocationEXT` allowing shaders to “demote” a fragment
//!shader invocation to behave like a helper invocation for its duration.
//!The demoted invocation will have no further side effects and will not output
//!to the framebuffer, but remains active and can participate in computing
//!derivatives and in [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations).
//!This is a better match for the “discard” instruction in HLSL.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_demote_to_helper_invocation]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_shader_demote_to_helper_invocation extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`]
//!# New constants
//! - [`EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME`]
//! - [`EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-06-01 (Jeff Bolz)  - Initial draft
//!# Other info
//! * 2019-06-01
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - This extension requires [`SPV_EXT_demote_to_helper_invocation`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_demote_to_helper_invocation.html)
//!   - This extension provides API support for [`GL_EXT_demote_to_helper_invocation`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_demote_to_helper_invocation.txt)
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT`]
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
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_shader_demote_to_helper_invocation");
