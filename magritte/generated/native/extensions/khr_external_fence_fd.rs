use crate::native::{
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportFenceFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub fd: i32,
}
impl Default for ImportFenceFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportFenceFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fence: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            fd: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FenceGetFdInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub fence: Fence,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl Default for FenceGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::FenceGetFdInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fence: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_fence_fd::{
    KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME, KHR_EXTERNAL_FENCE_FD_SPEC_VERSION,
};
#[doc(alias = "vkGetFenceFdKHR")]
pub type FNGetFenceFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const FenceGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
#[doc(alias = "vkImportFenceFdKHR")]
pub type FNImportFenceFdKhr =
    unsafe extern "system" fn(device: Device, p_import_fence_fd_info: *const ImportFenceFdInfoKHR) -> VulkanResultCodes;
