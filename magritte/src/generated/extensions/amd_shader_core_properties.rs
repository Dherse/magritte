//![VK_AMD_shader_core_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties.html) - device extension
//!# Description
//!This extension exposes shader core properties for a target physical device
//!through the `[`VK_KHR_get_physical_device_properties2`]` extension.
//!Please refer to the example below for proper usage.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Martin Dinkov [mdinkov](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_core_properties]
//!   @mdinkov%0A<<Here describe the issue or question you have about the
//!   VK_AMD_shader_core_properties extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceShaderCorePropertiesAMD`]
//!# New constants
//! - [`AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME`]
//! - [`AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`
//!# Version History
//! - Revision 2, 2019-06-25 (Matthaeus G. Chajdas)
//! - Clarified the meaning of a few fields.
//! - Revision 1, 2018-02-15 (Martin Dinkov)
//! - Initial draft.
//!# Other info
//! * 2019-06-25
//! * No known IP claims.
//!*
//! - Martin Dinkov, AMD
//! - Matthaeus G. Chajdas, AMD
//!# Related
//! - [`PhysicalDeviceShaderCorePropertiesAMD`]
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
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_core_properties");
