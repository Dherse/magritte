//![VK_INTEL_shader_integer_functions2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_INTEL_shader_integer_functions2.html) - device extension
//!# Description
//!This extension adds support for several new integer instructions in SPIR-V
//!for use in graphics shaders.
//!Many of these instructions have pre-existing counterparts in the Kernel
//!environment.The added integer functions are defined by the
//![`SPV_INTEL_shader_integer_functions2`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/INTEL/SPV_INTEL_shader_integer_functions2.html)
//!SPIR-V extension and can be used with the GL_INTEL_shader_integer_functions2
//!GLSL extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Ian Romanick [ianromanick](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_INTEL_shader_integer_functions2]
//!   @ianromanick%0A<<Here describe the issue or question you have about the
//!   VK_INTEL_shader_integer_functions2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`]
//!# New constants
//! - [`INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME`]
//! - [`INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`
//!# Version History
//! - Revision 1, 2019-04-30 (Ian Romanick)
//! - Initial draft
//!# Other info
//! * 2019-04-30
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_INTEL_shader_integer_functions2`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/INTEL/SPV_INTEL_shader_integer_functions2.html).
//! - This extension provides API support for
//![`GL_INTEL_shader_integer_functions2`](https://www.khronos.org/registry/OpenGL/extensions/INTEL/INTEL_shader_integer_functions2.txt).
//!*
//! - Ian Romanick, Intel
//! - Ben Ashbaugh, Intel
//!# Related
//! - [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`]
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
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_INTEL_shader_integer_functions2");
