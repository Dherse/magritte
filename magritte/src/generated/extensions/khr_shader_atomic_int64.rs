//![VK_KHR_shader_atomic_int64](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_atomic_int64.html) - device extension
//!# Description
//!This extension advertises the SPIR-V  **Int64Atomics**  capability for Vulkan,
//!which allows a shader to contain 64-bit atomic operations on signed and
//!unsigned integers.
//!The supported operations include OpAtomicMin, OpAtomicMax, OpAtomicAnd,
//!OpAtomicOr, OpAtomicXor, OpAtomicAdd, OpAtomicExchange, and
//!OpAtomicCompareExchange.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Aaron Hagan [ahagan](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_atomic_int64]
//!   @ahagan%0A<<Here describe the issue or question you have about the VK_KHR_shader_atomic_int64
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderAtomicInt64FeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME`]
//! - [`KHR_SHADER_ATOMIC_INT64_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2018-07-05 (Aaron Hagan)  - Internal revisions
//!# Other info
//! * 2018-07-05
//! * - Promoted to Vulkan 1.2 Core  - This extension provides API support for [`GL_ARB_gpu_shader_int64`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_gpu_shader_int64.txt)
//!   and [`GL_EXT_shader_atomic_int64`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_atomic_int64.txt)
//! * - Aaron Hagan, AMD  - Daniel Rakos, AMD  - Jeff Bolz, NVIDIA  - Neil Henning, Codeplay
//!# Related
//! - [`PhysicalDeviceShaderAtomicInt64FeaturesKHR`]
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
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION")]
pub const KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME")]
pub const KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_atomic_int64");
