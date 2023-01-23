//!# [VK_KHR_pipeline_library](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_library.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_library/VK_KHR_pipeline_library.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Pipeline, StructureType},
};
use std::ffi::CStr;
///# [VkPipelineLibraryCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_pipeline_library/VkPipelineLibraryCreateInfoKHR.md")]
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "libraryCount")]
    library_count: u32,
    #[doc(alias = "pLibraries")]
    libraries: *const Pipeline,
}
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_pipeline_library");
