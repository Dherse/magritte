//![VK_KHR_shader_float16_int8](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_float16_int8.html) - device extension
//!# Description
//!The [`VK_KHR_shader_float16_int8`] extension allows use of 16-bit
//!floating-point types and 8-bit integer types in shaders for arithmetic
//!operations.It introduces two new optional features `shaderFloat16` and
//!`shaderInt8` which directly map to the `Float16` and the `Int8`
//!SPIR-V capabilities.
//!The [`VK_KHR_shader_float16_int8`] extension also specifies precision
//!requirements for half-precision floating-point SPIR-V operations.
//!This extension does not enable use of 8-bit integer types or 16-bit
//!floating-point types in any [shader input and
//!output interfaces](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-iointerfaces) and therefore does not supersede the
//!`[`khr_8bit_storage`]` or `[`khr_16bit_storage`]` extensions.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Alexander Galazin [alegal-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_float16_int8]
//!   @alegal-arm%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_float16_int8 extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceFloat16Int8FeaturesKHR`]  - [`PhysicalDeviceShaderFloat16Int8FeaturesKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME`]
//! - [`KHR_SHADER_FLOAT16_INT8_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2018-03-07 (Alexander Galazin)  - Initial draft
//!# Other info
//! * 2018-03-07
//! *   - Promoted to Vulkan 1.2 Core  - This extension interacts with `[`khr_8bit_storage`]`  - This extension interacts with `[`khr_16bit_storage`]`  - This extension interacts with `[`khr_shader_float_controls`]`  - This extension provides API support for [`GL_EXT_shader_explicit_arithmetic_types`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_explicit_arithmetic_types.txt)
//! * No known IP claims.
//! * - Alexander Galazin, Arm  - Jan-Harald Fredriksen, Arm  - Jeff Bolz, NVIDIA  - Graeme Leese,
//!   Broadcom  - Daniel Rakos, AMD
//!# Related
//! - [`PhysicalDeviceFloat16Int8FeaturesKHR`]
//! - [`PhysicalDeviceShaderFloat16Int8FeaturesKHR`]
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
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_float16_int8");
