//!# [VK_EXT_external_memory_host](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_host.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_external_memory_host/VK_EXT_external_memory_host.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceSize, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::ffi::CStr;
///# [VkImportMemoryHostPointerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_external_memory_host/VkImportMemoryHostPointerInfoEXT.md")]
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
    #[doc(alias = "pHostPointer")]
    host_pointer: *mut std::ffi::c_void,
}
///# [VkMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_external_memory_host/VkMemoryHostPointerPropertiesEXT.md")]
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
}
///# [VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_external_memory_host/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "minImportedHostPointerAlignment")]
    min_imported_host_pointer_alignment: DeviceSize,
}
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_external_memory_host");
///# [vkGetMemoryHostPointerPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_external_memory_host/vkGetMemoryHostPointerPropertiesEXT.md")]
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
pub type FNGetMemoryHostPointerPropertiesExt = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    p_host_pointer: *const std::ffi::c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> VulkanResultCodes;
