//![VK_KHR_vulkan_memory_model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_vulkan_memory_model.html) - device extension
//!# Description
//!The [`VK_KHR_vulkan_memory_model`] extension allows use of the
//![Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model), which formally defines how to
//!synchronize memory accesses to the same memory locations performed by
//!multiple shader invocations.
//!# Revision
//!3
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_vulkan_memory_model]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_vulkan_memory_model extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceVulkanMemoryModelFeaturesKHR`]
//!# New constants
//! - [`KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME`]
//! - [`KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2018-06-24 (Jeff Bolz)
//! - Initial draft
//! - Revision 3, 2018-12-10 (Jeff Bolz)
//! - Add vulkanMemoryModelAvailabilityVisibilityChains member to the
//!VkPhysicalDeviceVulkanMemoryModelFeaturesKHR structure.
//!# Other info
//! * 2018-12-10
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.2 Core
//! - This extension requires
//![`SPV_KHR_vulkan_memory_model`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_vulkan_memory_model.html)
//!*
//! - Jeff Bolz, NVIDIA
//! - Alan Baker, Google
//! - Tobias Hector, AMD
//! - David Neto, Google
//! - Robert Simpson, Qualcomm Technologies, Inc.
//! - Brian Sumner, AMD
//!# Related
//! - [`PhysicalDeviceVulkanMemoryModelFeaturesKHR`]
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
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION")]
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME")]
pub const KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_vulkan_memory_model");
