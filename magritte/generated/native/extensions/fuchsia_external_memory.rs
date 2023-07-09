use crate::native::{
    opaque::zx_handle_t,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: zx_handle_t,
}
impl Default for ImportMemoryZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryZirconHandleInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            handle: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl Default for MemoryZirconHandlePropertiesFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryZirconHandlePropertiesFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetZirconHandleInfoFUCHSIA {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryGetZirconHandleInfoFuchsia,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::fuchsia_external_memory::{
    FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME, FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION,
};
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
