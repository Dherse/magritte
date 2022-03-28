//![VK_KHR_8bit_storage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_8bit_storage.html) - device extension
//!# Description
//!The [`VK_KHR_8bit_storage`] extension allows use of 8-bit types in uniform and
//!storage buffers, and push constant blocks.
//!This extension introduces several new optional features which map to SPIR-V
//!capabilities and allow access to 8-bit data in `Block`-decorated objects
//!in the `Uniform` and the `StorageBuffer` storage classes, and objects
//!in the `PushConstant` storage class.The `StorageBuffer8BitAccess` capability  **must**  be
//! supported by all
//!implementations of this extension.
//!The other capabilities are optional.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_storage_buffer_storage_class`]`
//!# Contacts
//! - Alexander Galazin [alegal-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_8bit_storage]
//!   @alegal-arm%0A<<Here describe the issue or question you have about the VK_KHR_8bit_storage
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevice8BitStorageFeaturesKHR`]
//!# New constants
//! - [`KHR_8BIT_STORAGE_EXTENSION_NAME`]
//! - [`KHR_8BIT_STORAGE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2018-02-05 (Alexander Galazin)  - Initial draft
//!# Other info
//! * 2018-02-05
//! * - Promoted to Vulkan 1.2 Core  - This extension requires [`SPV_KHR_8bit_storage`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_8bit_storage.html)
//!   - This extension provides API support for [`GL_EXT_shader_16bit_storage`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_16bit_storage.txt)
//! * No known IP claims.
//! * - Alexander Galazin, Arm
//!# Related
//! - [`PhysicalDevice8BitStorageFeaturesKHR`]
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
#[doc(alias = "VK_KHR_8BIT_STORAGE_SPEC_VERSION")]
pub const KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_8BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_8BIT_STORAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_8bit_storage");
