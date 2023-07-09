use crate::native::{
    opaque::zx_handle_t,
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    #[doc(alias = "zirconHandle")]
    pub zircon_handle: zx_handle_t,
}
impl Default for ImportSemaphoreZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportSemaphoreZirconHandleInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            zircon_handle: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for SemaphoreGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreGetZirconHandleInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::fuchsia_external_semaphore::{
    FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME, FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION,
};
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
pub type FNGetSemaphoreZirconHandleFuchsia = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut zx_handle_t,
) -> VulkanResultCodes;
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
pub type FNImportSemaphoreZirconHandleFuchsia = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VulkanResultCodes;
