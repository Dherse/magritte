use crate::native::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub fd: i32,
}
impl Default for ImportMemoryFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            fd: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl Default for MemoryFdPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryFdPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryGetFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_memory_fd::{
    KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION,
};
#[doc(alias = "vkGetMemoryFdKHR")]
pub type FNGetMemoryFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
pub type FNGetMemoryFdPropertiesKhr = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    fd: i32,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> VulkanResultCodes;
