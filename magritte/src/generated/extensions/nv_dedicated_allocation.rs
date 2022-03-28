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
//! - *Deprecated* by `[`VK_KHR_dedicated_allocation`]` extension  - Which in turn was *promoted* to
//!   [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_dedicated_allocation]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_dedicated_allocation extension>>)
//!# New structures
//! - Extending [`BufferCreateInfo`]:  - [`DedicatedAllocationBufferCreateInfoNV`]
//! - Extending [`ImageCreateInfo`]:  - [`DedicatedAllocationImageCreateInfoNV`]
//! - Extending [`MemoryAllocateInfo`]:  - [`DedicatedAllocationMemoryAllocateInfoNV`]
//!# New constants
//! - [`NV_DEDICATED_ALLOCATION_EXTENSION_NAME`]
//! - [`NV_DEDICATED_ALLOCATION_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
//!   - `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
//!# Version History
//! - Revision 1, 2016-05-31 (Jeff Bolz)  - Internal revisions
//!# Other info
//! * 2016-05-31
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA
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
///## Valid Usage
/// - If [`dedicated_allocation`] is [`TRUE`], [`ImageCreateInfo::flags`] **must**  not include
///   `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, or
///   `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`
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
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`dedicated_allocation`] specifies whether the image will have a
    ///dedicated allocation bound to it.
    pub dedicated_allocation: Bool32,
}
impl<'lt> Default for DedicatedAllocationImageCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            dedicated_allocation: 0,
        }
    }
}
impl<'lt> DedicatedAllocationImageCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation_raw(&self) -> Bool32 {
        self.dedicated_allocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dedicated_allocation`]
    pub fn set_dedicated_allocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.dedicated_allocation = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation(&self) -> bool {
        unsafe { std::mem::transmute(self.dedicated_allocation as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::dedicated_allocation`]
    pub fn set_dedicated_allocation(&mut self, value: bool) -> &mut Self {
        self.dedicated_allocation = value as u8 as u32;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV`
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
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`dedicated_allocation`] specifies whether the buffer will have a
    ///dedicated allocation bound to it.
    pub dedicated_allocation: Bool32,
}
impl<'lt> Default for DedicatedAllocationBufferCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            dedicated_allocation: 0,
        }
    }
}
impl<'lt> DedicatedAllocationBufferCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation_raw(&self) -> Bool32 {
        self.dedicated_allocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dedicated_allocation`]
    pub fn set_dedicated_allocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.dedicated_allocation = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation(&self) -> bool {
        unsafe { std::mem::transmute(self.dedicated_allocation as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dedicated_allocation`]
    pub fn dedicated_allocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::dedicated_allocation`]
    pub fn set_dedicated_allocation(&mut self, value: bool) -> &mut Self {
        self.dedicated_allocation = value as u8 as u32;
        self
    }
}
///[VkDedicatedAllocationMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html) - Specify a dedicated memory allocation resource
///# C Specifications
///If the [`p_next`] chain includes a
///[`DedicatedAllocationMemoryAllocateInfoNV`] structure, then that
///structure includes a handle of the sole buffer or image resource that the
///memory  **can**  be bound to.The [`DedicatedAllocationMemoryAllocateInfoNV`] structure is
/// defined as:
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
///## Valid Usage
/// - At least one of [`image`] and [`buffer`] **must**  be [`crate::utils::Handle::null`]
/// - If [`image`] is not [`crate::utils::Handle::null`], the image  **must**  have been created
///   with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
/// - If [`buffer`] is not [`crate::utils::Handle::null`], the buffer  **must**  have been created
///   with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`] equal to [`TRUE`]
/// - If [`image`] is not [`crate::utils::Handle::null`], [`MemoryAllocateInfo::allocation_size`]
///   **must**  equal the [`MemoryRequirements::size`] of the image
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`MemoryAllocateInfo::allocation_size`]
///   **must**  equal the [`MemoryRequirements::size`] of the buffer
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation, the memory being imported  **must**  also be a dedicated image allocation
///   and [`image`] **must**  be identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation, the memory being imported  **must**  also be a dedicated buffer
///   allocation and [`buffer`] **must**  be identical to the buffer associated with the imported
///   memory
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV`
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`] **must**  be a valid [`Image`]
///   handle
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`] **must**  be a valid
///   [`Buffer`] handle
/// - Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters  **must**
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
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image`] is [`crate::utils::Handle::null`] or a handle of an image which this
    ///memory will be bound to.
    pub image: Image,
    ///[`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this
    ///memory will be bound to.
    pub buffer: Buffer,
}
impl<'lt> Default for DedicatedAllocationMemoryAllocateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
impl<'lt> DedicatedAllocationMemoryAllocateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
}
