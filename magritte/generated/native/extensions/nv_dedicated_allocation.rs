use crate::native::vulkan1_0::{BaseInStructure, Bool32, Buffer, Image, StructureType};
#[doc(alias = "VkDedicatedAllocationImageCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dedicatedAllocation")]
    pub dedicated_allocation: Bool32,
}
impl Default for DedicatedAllocationImageCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::DedicatedAllocationImageCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            dedicated_allocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDedicatedAllocationBufferCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "dedicatedAllocation")]
    pub dedicated_allocation: Bool32,
}
impl Default for DedicatedAllocationBufferCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::DedicatedAllocationBufferCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            dedicated_allocation: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDedicatedAllocationMemoryAllocateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub image: Image,
    pub buffer: Buffer,
}
impl Default for DedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::DedicatedAllocationMemoryAllocateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            image: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_dedicated_allocation::{
    NV_DEDICATED_ALLOCATION_EXTENSION_NAME, NV_DEDICATED_ALLOCATION_SPEC_VERSION,
};
