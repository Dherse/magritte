//![VK_NV_external_memory_rdma](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_rdma.html) - device extension
//!# Description
//!This extension adds support for allocating memory which can be used for
//!remote direct memory access (RDMA) from other devices.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - Carsten Rohde [crohde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_rdma]
//!   @crohde%0A<<Here describe the issue or question you have about the VK_NV_external_memory_rdma
//!   extension>>)
//!# New functions & commands
//! - [`GetMemoryRemoteAddressNV`]
//!# New structures
//! - [`MemoryGetRemoteAddressInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:
//! - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV`
//! - Extending [`MemoryPropertyFlagBits`]:
//! - `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
//!# Version History
//! - Revision 1, 2020-12-15 (Carsten Rohde)
//! - Internal revisions
//!# Other info
//! * 2021-04-19
//! * No known IP claims.
//!*
//! - Carsten Rohde, NVIDIA
//!# Related
//! - [`MemoryGetRemoteAddressInfoNV`]
//! - [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]
//! - [`RemoteAddressNV`]
//! - [`GetMemoryRemoteAddressNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::{c_void, CStr};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory_rdma");
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VkRemoteAddressNV")]
pub type RemoteAddressNV = c_void;
