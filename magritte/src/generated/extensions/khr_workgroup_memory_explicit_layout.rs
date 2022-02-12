//![VK_KHR_workgroup_memory_explicit_layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_workgroup_memory_explicit_layout.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_KHR_workgroup_memory_explicit_layout`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_workgroup_memory_explicit_layout.html)
//!SPIR-V extension, which allows shaders to explicitly define the layout of
//!`Workgroup` storage class memory and create aliases between variables
//!from that storage class in a compute shader.The aliasing feature allows different “views” on the
//! same data, so the
//!shader can bulk copy data from another storage class using one type (e.g. an
//!array of large vectors), and then use the data with a more specific type.
//!It also enables reducing the amount of workgroup memory consumed by allowing
//!the shader to alias data whose lifetimes do not overlap.The explicit layout support and some
//! form of aliasing is also required for
//!layering OpenCL on top of Vulkan.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Caio Marcelo de Oliveira Filho [cmarcelo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_workgroup_memory_explicit_layout]
//!   @cmarcelo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_workgroup_memory_explicit_layout extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]
//!# New constants
//! - [`KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME`]
//! - [`KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2020-06-01 (Caio Marcelo de Oliveira Filho)
//! - Initial version
//!# Other info
//! * 2020-06-01
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_KHR_workgroup_memory_explicit_layout`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_workgroup_memory_explicit_layout.html)
//! - This extension provides API support for
//![`GL_EXT_shared_memory_block`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shared_memory_block.txt)
//!*
//! - Caio Marcelo de Oliveira Filho, Intel
//! - Jeff Bolz, NVIDIA
//! - Graeme Leese, Broadcom
//! - Jason Ekstrand, Intel
//! - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]
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
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_workgroup_memory_explicit_layout");
