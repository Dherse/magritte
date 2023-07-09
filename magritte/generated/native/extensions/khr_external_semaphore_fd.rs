use crate::native::{
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub fd: i32,
}
impl Default for ImportSemaphoreFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportSemaphoreFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            fd: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for SemaphoreGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreGetFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_semaphore_fd::{
    KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION,
};
#[doc(alias = "vkGetSemaphoreFdKHR")]
pub type FNGetSemaphoreFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
#[doc(alias = "vkImportSemaphoreFdKHR")]
pub type FNImportSemaphoreFdKhr = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> VulkanResultCodes;
