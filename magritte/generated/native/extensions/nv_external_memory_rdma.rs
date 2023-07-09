use crate::native::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryRDMA")]
    pub external_memory_rdma: Bool32,
}
impl Default for PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalMemoryRdmaFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            external_memory_rdma: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetRemoteAddressInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryGetRemoteAddressInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_external_memory_rdma::{
    RemoteAddressNV, NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME, NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION,
};
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
pub type FNGetMemoryRemoteAddressNv = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    p_address: *mut RemoteAddressNV,
) -> VulkanResultCodes;
