//![VK_KHR_get_memory_requirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_memory_requirements2.html) - device extension
//!# Description
//!This extension provides new entry points to query memory requirements of
//!images and buffers in a way that can be easily extended by other extensions,
//!without introducing any further entry points.
//!The Vulkan 1.0 [`MemoryRequirements`] and
//![`SparseImageMemoryRequirements`] structures do not include `sType`
//!and `pNext` members.
//!This extension wraps them in new structures with these members, so an
//!application can query a chain of memory requirements structures by
//!constructing the chain and letting the implementation fill them in.
//!A new command is added for each `vkGet*MemoryRequrements` command in
//!core Vulkan 1.0.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_memory_requirements2]
//!   @jekstrand%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_memory_requirements2 extension>>)
//!# New functions & commands
//! - [`GetBufferMemoryRequirements2KHR`]
//! - [`GetImageMemoryRequirements2KHR`]
//! - [`GetImageSparseMemoryRequirements2KHR`]
//!# New structures
//! - [`BufferMemoryRequirementsInfo2KHR`]
//! - [`ImageMemoryRequirementsInfo2KHR`]
//! - [`ImageSparseMemoryRequirementsInfo2KHR`]
//! - [`MemoryRequirements2KHR`]
//! - [`SparseImageMemoryRequirements2KHR`]
//!# New constants
//! - [`KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME`]
//! - [`KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR`
//! - `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR`
//! - `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR`
//! - `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR`
//! - `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR`
//!# Version History
//! - Revision 1, 2017-03-23 (Jason Ekstrand)
//! - Internal revisions
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!*
//! - Jason Ekstrand, Intel
//! - Jeff Bolz, NVIDIA
//! - Jesse Hall, Google
//!# Related
//! - [`BufferMemoryRequirementsInfo2KHR`]
//! - [`ImageMemoryRequirementsInfo2KHR`]
//! - [`ImageSparseMemoryRequirementsInfo2KHR`]
//! - [`MemoryRequirements2KHR`]
//! - [`SparseImageMemoryRequirements2KHR`]
//! - [`GetBufferMemoryRequirements2KHR`]
//! - [`GetImageMemoryRequirements2KHR`]
//! - [`GetImageSparseMemoryRequirements2KHR`]
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
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_get_memory_requirements2");
