//![VK_KHR_shader_float_controls](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_float_controls.html) - device extension
//!# Description
//!The [`VK_KHR_shader_float_controls`] extension enables efficient use of
//!floating-point computations through the ability to query and override the
//!implementation’s default behavior for rounding modes, denormals, signed
//!zero, and infinity.
# ! [doc = concat ! ("# " , "Revision")]
//!4
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Alexander Galazin [alegal-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_float_controls]
//!   @alegal-arm%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_float_controls extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceFloatControlsPropertiesKHR`]
# ! [doc = concat ! ("# " , "New enums")]
//! - [`ShaderFloatControlsIndependenceKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME`]
//! - [`KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION`]
//! - Extending [`ShaderFloatControlsIndependence`]:  -
//!   `VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR`  -
//!   `VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR`  -
//!   `VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!1) Which instructions must flush denorms? **RESOLVED** : Only floating-point conversion,
//! floating-point arithmetic,
//!floating-point relational (except `OpIsNaN`, `OpIsInf`), and
//!floating-point GLSL.std.450 extended instructions must flush denormals.2) What is the denorm
//! behavior for intermediate results? **RESOLVED** : When a SPIR-V instruction is implemented as a
//! sequence of other
//!instructions:
//! - in the `DenormFlushToZero` execution mode, the intermediate instructions may flush denormals,
//!   the final result of the sequence  **must**  not be denormal.
//! - in the `DenormPreserve` execution mode, denormals must be preserved throughout the whole
//!   sequence.
//!3) Do denorm and rounding mode controls apply to `OpSpecConstantOp`? **RESOLVED** : Yes, except
//! when the opcode is `OpQuantizeToF16`.4) The SPIR-V specification says that `OpConvertFToU` and
//!`OpConvertFToS` unconditionally round towards zero.
//!Do the rounding mode controls specified through the execution modes apply to
//!them? **RESOLVED** : No, these instructions unconditionally round towards zero.5) Do any of the
//! “Pack” GLSL.std.450 instructions count as conversion
//!instructions and have the rounding mode applied? **RESOLVED** : No, only instructions listed in
//! “section 3.32.11.
//!Conversion Instructions” of the SPIR-V specification count as conversion
//!instructions.6) When using inf/nan-ignore mode, what is expected of `OpIsNan` and
//!`OpIsInf`? **RESOLVED** : These instructions must always accurately detect inf/nan if it
//!is passed to them.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 4, 2019-06-18 (Tobias Hector)  - Modified settings restrictions, see [Version 4 API
//!   incompatibility]()
//! - Revision 3, 2018-09-11 (Alexander Galazin)  - Minor restructuring
//! - Revision 2, 2018-04-17 (Alexander Galazin)  - Added issues and resolutions
//! - Revision 1, 2018-04-11 (Alexander Galazin)  - Initial draft
//!# Other info
//! * 2018-09-11
//! * - Promoted to Vulkan 1.2 Core  - This extension requires [`SPV_KHR_float_controls`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_float_controls.html)
//! * No known IP claims.
//! * - Alexander Galazin, Arm  - Jan-Harald Fredriksen, Arm  - Jeff Bolz, NVIDIA  - Graeme Leese,
//!   Broadcom  - Daniel Rakos, AMD
//!# Related
//! - [`PhysicalDeviceFloatControlsPropertiesKHR`]
//! - [`ShaderFloatControlsIndependenceKHR`]
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
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_float_controls");
