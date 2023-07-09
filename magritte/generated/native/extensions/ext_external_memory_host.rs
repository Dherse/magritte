use crate::native::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceSize, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    #[doc(alias = "pHostPointer")]
    pub host_pointer: *mut std::ffi::c_void,
}
impl Default for ImportMemoryHostPointerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryHostPointerInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            host_pointer: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl Default for MemoryHostPointerPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryHostPointerPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minImportedHostPointerAlignment")]
    pub min_imported_host_pointer_alignment: DeviceSize,
}
impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExternalMemoryHostPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            min_imported_host_pointer_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_external_memory_host::{
    EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME, EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION,
};
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
pub type FNGetMemoryHostPointerPropertiesExt = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    p_host_pointer: *const std::ffi::c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> VulkanResultCodes;
