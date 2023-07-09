use crate::native::vulkan1_0::{BaseInStructure, Pipeline, StructureType};
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "libraryCount")]
    pub library_count: u32,
    #[doc(alias = "pLibraries")]
    pub libraries: *const Pipeline,
}
impl Default for PipelineLibraryCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineLibraryCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            library_count: unsafe { std::mem::zeroed() },
            libraries: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_pipeline_library::{
    KHR_PIPELINE_LIBRARY_EXTENSION_NAME, KHR_PIPELINE_LIBRARY_SPEC_VERSION,
};
