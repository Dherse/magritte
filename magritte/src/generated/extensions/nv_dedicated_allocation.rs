use crate::vulkan1_0::{BaseInStructure, Bool32, Buffer, Image, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_dedicated_allocation");
///[VkDedicatedAllocationImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) - Specify that an image is bound to a dedicated memory resource
///# C Specifications
///If the [`p_next`] chain includes a
///[`DedicatedAllocationImageCreateInfoNV`] structure, then that structure
///includes an enable controlling whether the image will have a dedicated
///memory allocation bound to it.The [`DedicatedAllocationImageCreateInfoNV`] structure is defined
/// as:
///```c
///// Provided by VK_NV_dedicated_allocation
///typedef struct VkDedicatedAllocationImageCreateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           dedicatedAllocation;
///} VkDedicatedAllocationImageCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`dedicated_allocation`] specifies whether the image will have a dedicated allocation bound to
///   it.
///# Description
///Valid Usage
/// - If [`dedicated_allocation`] is [`TRUE`], [`ImageCreateInfo::flags`]**must** not include
///   `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, or
///   `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_dedicated_allocation`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`dedicated_allocation`] specifies whether the image will have a
    ///dedicated allocation bound to it.
    dedicated_allocation: Bool32,
}
///[VkDedicatedAllocationBufferCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html) - Specify that a buffer is bound to a dedicated memory resource
///# C Specifications
///If the [`p_next`] chain includes a
///[`DedicatedAllocationBufferCreateInfoNV`] structure, then that structure
///includes an enable controlling whether the buffer will have a dedicated
///memory allocation bound to it.The [`DedicatedAllocationBufferCreateInfoNV`] structure is defined
/// as:
///```c
///// Provided by VK_NV_dedicated_allocation
///typedef struct VkDedicatedAllocationBufferCreateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           dedicatedAllocation;
///} VkDedicatedAllocationBufferCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`dedicated_allocation`] specifies whether the buffer will have a dedicated allocation bound
///   to it.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_dedicated_allocation`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`dedicated_allocation`] specifies whether the buffer will have a
    ///dedicated allocation bound to it.
    dedicated_allocation: Bool32,
}
///[VkDedicatedAllocationMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) - Specify a dedicated memory allocation resource
///# C Specifications
///If the [`p_next`] chain includes a
///[`DedicatedAllocationMemoryAllocateInfoNV`] structure, then that
///structure includes a handle of the sole buffer or image resource that the
///memory **can** be bound to.The [`DedicatedAllocationMemoryAllocateInfoNV`] structure is defined
/// as:
///```c
///// Provided by VK_NV_dedicated_allocation
///typedef struct VkDedicatedAllocationMemoryAllocateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImage            image;
///    VkBuffer           buffer;
///} VkDedicatedAllocationMemoryAllocateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image`] is [`crate::utils::Handle::null`] or a handle of an image which this memory will be
///   bound to.
/// - [`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this memory will be
///   bound to.
///# Description
///Valid Usage
/// - At least one of [`image`] and [`buffer`]**must** be [`crate::utils::Handle::null`]
/// - If [`image`] is not [`crate::utils::Handle::null`], the image **must** have been created with
///   [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
/// - If [`buffer`] is not [`crate::utils::Handle::null`], the buffer **must** have been created
///   with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
/// - If [`image`] is not [`crate::utils::Handle::null`],
///   [`MemoryAllocateInfo::allocation_size`]**must** equal the [`MemoryRequirements::size`] of the
///   image
/// - If [`buffer`] is not [`crate::utils::Handle::null`],
///   [`MemoryAllocateInfo::allocation_size`]**must** equal the [`MemoryRequirements::size`] of the
///   buffer
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation, the memory being imported **must** also be a dedicated image allocation and
///   [`image`]**must** be identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation, the memory being imported **must** also be a dedicated buffer
///   allocation and [`buffer`]**must** be identical to the buffer associated with the imported
///   memory
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`]**must** be a valid [`Image`]
///   handle
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`]**must** be a valid [`Buffer`]
///   handle
/// - Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters **must**
///   have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`VK_NV_dedicated_allocation`]
/// - [`Buffer`]
/// - [`Image`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image`] is [`crate::utils::Handle::null`] or a handle of an image which this
    ///memory will be bound to.
    image: Image,
    ///[`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this
    ///memory will be bound to.
    buffer: Buffer,
}
