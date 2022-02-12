//![VK_NV_dedicated_allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation.html) - device extension
//!# Description
//!This extension allows device memory to be allocated for a particular buffer
//!or image resource, which on some devices can significantly improve the
//!performance of that resource.
//!Normal device memory allocations must support memory aliasing and sparse
//!binding, which could interfere with optimizations like framebuffer
//!compression or efficient page table usage.
//!This is important for render targets and very large resources, but need not
//!(and probably should not) be used for smaller resources that can benefit
//!from suballocation.This extension adds a few small structures to resource creation and memory
//!allocation: a new structure that flags whether am image/buffer will have a
//!dedicated allocation, and a structure indicating the image or buffer that an
//!allocation will be bound to.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by
//!`[`VK_KHR_dedicated_allocation`]`
//!extension
//! - Which in turn was *promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_dedicated_allocation]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_dedicated_allocation extension>>)
//!# New structures
//! - Extending [`BufferCreateInfo`]:
//! - [`DedicatedAllocationBufferCreateInfoNV`]
//!
//! - Extending [`ImageCreateInfo`]:
//! - [`DedicatedAllocationImageCreateInfoNV`]
//!
//! - Extending [`MemoryAllocateInfo`]:
//! - [`DedicatedAllocationMemoryAllocateInfoNV`]
//!# New constants
//! - [`NV_DEDICATED_ALLOCATION_EXTENSION_NAME`]
//! - [`NV_DEDICATED_ALLOCATION_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
//!# Version History
//! - Revision 1, 2016-05-31 (Jeff Bolz)
//! - Internal revisions
//!# Other info
//! * 2016-05-31
//! * No known IP claims.
//!*
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`DedicatedAllocationBufferCreateInfoNV`]
//! - [`DedicatedAllocationImageCreateInfoNV`]
//! - [`DedicatedAllocationMemoryAllocateInfoNV`]
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
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_dedicated_allocation");
