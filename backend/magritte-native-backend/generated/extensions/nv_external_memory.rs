use crate::{
    cstr,
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_external_memory");
