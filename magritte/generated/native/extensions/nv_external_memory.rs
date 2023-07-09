use crate::native::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    vulkan1_0::{BaseInStructure, StructureType},
};
#[doc(alias = "VkExternalMemoryImageCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalMemoryImageCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExternalMemoryImageCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportMemoryAllocateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleTypes")]
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExportMemoryAllocateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportMemoryAllocateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            handle_types: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_external_memory::{
    NV_EXTERNAL_MEMORY_EXTENSION_NAME, NV_EXTERNAL_MEMORY_SPEC_VERSION,
};
