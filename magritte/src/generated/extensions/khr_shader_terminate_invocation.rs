//![VK_KHR_shader_terminate_invocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_terminate_invocation.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_KHR_terminate_invocation`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_terminate_invocation.html)
//!SPIR-V extension.
//!That SPIR-V extension provides a new instruction,
//!`OpTerminateInvocation`, which causes a shader invocation to immediately
//!terminate and sets the coverage of shaded samples to `0`; only previously
//!executed instructions will have observable effects.
//!The `OpTerminateInvocation` instruction, along with the
//!`OpDemoteToHelperInvocation` instruction from the
//!`[`VK_EXT_shader_demote_to_helper_invocation`]` extension, together
//!replace the `OpKill` instruction, which could behave like either of these
//!instructions.
//!`OpTerminateInvocation` provides the behavior required by the GLSL
//!`discard` statement, and should be used when available by GLSL compilers
//!and applications that need the GLSL `discard` behavior.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_terminate_invocation]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_terminate_invocation extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShaderTerminateInvocationFeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME`]
//! - [`KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2020-08-11 (Jesse Hall)
//!# Other info
//! * 2020-08-11
//!*
//! - Promoted to Vulkan 1.3 Core
//! - Requires the
//![`SPV_KHR_terminate_invocation`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_terminate_invocation.html)
//!SPIR-V extension.
//! * No known IP claims.
//!*
//! - Alan Baker, Google
//! - Jeff Bolz, NVIDIA
//! - Jesse Hall, Google
//! - Ralph Potter, Samsung
//! - Tom Olson, Arm
//!# Related
//! - [`PhysicalDeviceShaderTerminateInvocationFeaturesKHR`]
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
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION")]
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME")]
pub const KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_shader_terminate_invocation");
