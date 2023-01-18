use crate::{
    cstr,
    opaque::zx_handle_t,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::ffi::CStr;
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
    handle: zx_handle_t,
}
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
}
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    memory: DeviceMemory,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_FUCHSIA_external_memory");
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
pub type FNGetMemoryZirconHandleFuchsia = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut zx_handle_t,
) -> VulkanResultCodes;
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
pub type FNGetMemoryZirconHandlePropertiesFuchsia = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    zircon_handle: zx_handle_t,
    p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> VulkanResultCodes;
