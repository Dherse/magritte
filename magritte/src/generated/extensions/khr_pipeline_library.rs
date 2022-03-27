use crate::vulkan1_0::{BaseInStructure, Pipeline, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_pipeline_library");
///[VkPipelineLibraryCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) - Structure specifying pipeline libraries to use when creating a pipeline
///# C Specifications
///The [`PipelineLibraryCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_library
///typedef struct VkPipelineLibraryCreateInfoKHR {
///    VkStructureType      sType;
///    const void*          pNext;
///    uint32_t             libraryCount;
///    const VkPipeline*    pLibraries;
///} VkPipelineLibraryCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`library_count`] is the number of pipeline libraries in [`p_libraries`].
/// - [`p_libraries`] is a pointer to an array of [`Pipeline`] structures specifying pipeline
///   libraries to use when creating a pipeline.
///# Description
///Valid Usage
/// - Each element of [`p_libraries`]**must** have been created with
///   `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - If [`library_count`] is not `0`, [`p_libraries`]**must** be a valid pointer to an array of
///   [`library_count`] valid [`Pipeline`] handles
///# Related
/// - [`VK_KHR_pipeline_library`]
/// - [`Pipeline`]
/// - [`RayTracingPipelineCreateInfoKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`library_count`] is the number of pipeline libraries in
    ///[`p_libraries`].
    library_count: u32,
    ///[`p_libraries`] is a pointer to an array of [`Pipeline`] structures
    ///specifying pipeline libraries to use when creating a pipeline.
    p_libraries: *mut Pipeline,
}
