//![VK_KHR_16bit_storage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_16bit_storage.html) - device extension
//!# Description
//!The [`VK_KHR_16bit_storage`] extension allows use of 16-bit types in shader
//!input and output interfaces, and push constant blocks.
//!This extension introduces several new optional features which map to SPIR-V
//!capabilities and allow access to 16-bit data in `Block`-decorated objects
//!in the `Uniform` and the `StorageBuffer` storage classes, and objects
//!in the `PushConstant` storage class.
//!This extension allows 16-bit variables to be declared and used as
//!user-defined shader inputs and outputs but does not change location
//!assignment and component assignment rules.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`khr_storage_buffer_storage_class`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_16bit_storage]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_KHR_16bit_storage extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevice16BitStorageFeaturesKHR`]
//!# New constants
//! - [`KHR_16BIT_STORAGE_EXTENSION_NAME`]
//! - [`KHR_16BIT_STORAGE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2017-03-23 (Alexander Galazin)  - Initial draft
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core  - This extension requires [`SPV_KHR_16bit_storage`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_16bit_storage.html)
//!   - This extension provides API support for [`GL_EXT_shader_16bit_storage`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_16bit_storage.txt)
//! * - Alexander Galazin, ARM  - Jan-Harald Fredriksen, ARM  - Joerg Wagner, ARM  - Neil Henning,
//!   Codeplay  - Jeff Bolz, Nvidia  - Daniel Koch, Nvidia  - David Neto, Google  - John Kessenich,
//!   Google
//!# Related
//! - [`PhysicalDevice16BitStorageFeaturesKHR`]
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
#[doc(alias = "VK_KHR_16BIT_STORAGE_SPEC_VERSION")]
pub const KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_16BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_16BIT_STORAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_16bit_storage");
