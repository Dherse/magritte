//![VK_KHR_dedicated_allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_dedicated_allocation.html) - device extension
//!# Description
//!This extension enables resources to be bound to a dedicated allocation,
//!rather than suballocated.
//!For any particular resource, applications **can** query whether a dedicated
//!allocation is recommended, in which case using a dedicated allocation **may**
//!improve the performance of access to that resource.
//!Normal device memory allocations must support multiple resources per
//!allocation, memory aliasing and sparse binding, which could interfere with
//!some optimizations.
//!Applications should query the implementation for when a dedicated allocation
//!**may** be beneficial by adding a [`MemoryDedicatedRequirementsKHR`]
//!structure to the `pNext` chain of the [`MemoryRequirements2`]
//!structure passed as the `pMemoryRequirements` parameter of a call to
//![`GetBufferMemoryRequirements2`] or [`GetImageMemoryRequirements2`].
//!Certain external handle types and external images or buffers **may** also
//!depend on dedicated allocations on implementations that associate image or
//!buffer metadata with OS-level memory objects.This extension adds a two small structures to
//! memory requirements querying
//!and memory allocation: a new structure that flags whether an image/buffer
//!should have a dedicated allocation, and a structure indicating the image or
//!buffer that an allocation will be bound to.
//!# Revision
//!3
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_memory_requirements2`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_dedicated_allocation]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_dedicated_allocation extension>>)
//!# New structures
//! - Extending [`MemoryAllocateInfo`]:
//! - [`MemoryDedicatedAllocateInfoKHR`]
//!
//! - Extending [`MemoryRequirements2`]:
//! - [`MemoryDedicatedRequirementsKHR`]
//!# New constants
//! - [`KHR_DEDICATED_ALLOCATION_EXTENSION_NAME`]
//! - [`KHR_DEDICATED_ALLOCATION_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR`
//!# Version History
//! - Revision 1, 2017-02-27 (James Jones)
//! - Copy content from VK_NV_dedicated_allocation
//! - Add some references to external object interactions to the overview.
//!
//! - Revision 2, 2017-03-27 (Jason Ekstrand)
//! - Rework the extension to be query-based
//!
//! - Revision 3, 2017-07-31 (Jason Ekstrand)
//! - Clarify that memory objects allocated with
//!VkMemoryDedicatedAllocateInfoKHR can only have the specified resource
//!bound and no others.
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!
//!*
//! - Jeff Bolz, NVIDIA
//! - Jason Ekstrand, Intel
//!# Related
//! - [`MemoryDedicatedAllocateInfoKHR`]
//! - [`MemoryDedicatedRequirementsKHR`]
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
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_dedicated_allocation");
