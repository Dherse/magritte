//![VK_KHR_shader_clock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html) - device extension
//!# Description
//!This extension advertises the SPIR-V `ShaderClockKHR` capability for
//!Vulkan, which allows a shader to query a real-time or monotonically
//!incrementing counter at the subgroup level or across the device level.
//!The two valid SPIR-V scopes for `OpReadClockKHR` are `Subgroup` and
//![`Device`].When using GLSL source-based shading languages, the `clockRealtime*EXT`()
//!timing functions map to the `OpReadClockKHR` instruction with a scope of
//![`Device`], and the `clock*ARB`() timing functions map to the
//!`OpReadClockKHR` instruction with a scope of `Subgroup`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Aaron Hagan [ahagan](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_clock]
//!   @ahagan%0A<<Here describe the issue or question you have about the VK_KHR_shader_clock
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShaderClockFeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_CLOCK_EXTENSION_NAME`]
//! - [`KHR_SHADER_CLOCK_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2019-4-25 (Aaron Hagan)
//! - Initial revision
//!# Other info
//! * 2019-4-25
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_KHR_shader_clock`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_shader_clock.html).
//! - This extension provides API support for
//![`ARB_shader_clock`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_clock.txt) and
//![`EXT_shader_realtime_clock`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_realtime_clock.txt)
//!
//!*
//! - Aaron Hagan, AMD
//! - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderClockFeaturesKHR`]
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
#[doc(alias = "VK_KHR_SHADER_CLOCK_SPEC_VERSION")]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_CLOCK_EXTENSION_NAME")]
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_clock");
