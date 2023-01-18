use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRdmaFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "externalMemoryRDMA")]
    external_memory_rdma: Bool32,
}
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    memory: DeviceMemory,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[doc(alias = "VkRemoteAddressNV")]
pub type RemoteAddressNV = std::ffi::c_void;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_external_memory_rdma");
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
pub type FNGetMemoryRemoteAddressNv = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    p_address: *mut RemoteAddressNV,
) -> VulkanResultCodes;
