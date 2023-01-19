//!# [VK_NV_dedicated_allocation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation/VK_NV_dedicated_allocation.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, Buffer, Image, StructureType},
};
use std::ffi::CStr;
///# [VkDedicatedAllocationImageCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation/VkDedicatedAllocationImageCreateInfoNV.md")]
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "dedicatedAllocation")]
    dedicated_allocation: Bool32,
}
///# [VkDedicatedAllocationBufferCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationBufferCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation/VkDedicatedAllocationBufferCreateInfoNV.md")]
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "dedicatedAllocation")]
    dedicated_allocation: Bool32,
}
///# [VkDedicatedAllocationMemoryAllocateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation/VkDedicatedAllocationMemoryAllocateInfoNV.md")]
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    image: Image,
    buffer: Buffer,
}
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_dedicated_allocation");
