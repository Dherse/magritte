//![VK_KHR_pipeline_library](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_library.html) - device extension
//!# Description
//!A pipeline library is a special pipeline that cannot be bound, instead it
//!defines a set of shaders and shader groups which can be linked into other
//!pipelines.
//!This extension defines the infrastructure for pipeline libraries, but does
//!not specify the creation or usage of pipeline libraries.
//!This is left to additional dependent extensions.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Christoph Kubisch [pixeljetstream](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_pipeline_library]
//!   @pixeljetstream%0A<<Here describe the issue or question you have about the
//!   VK_KHR_pipeline_library extension>>)
//!# New structures
//! - [`PipelineLibraryCreateInfoKHR`]
//!# New constants
//! - [`KHR_PIPELINE_LIBRARY_EXTENSION_NAME`]
//! - [`KHR_PIPELINE_LIBRARY_SPEC_VERSION`]
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
//!# Version history
//! - Revision 1, 2020-01-08 (Christoph Kubisch)  - Initial draft.
//!# Other information
//! * 2020-01-08
//! * No known IP claims.
//! * - See contributors to `[`khr_ray_tracing_pipeline`]`
//!# Related
//! - [`PipelineLibraryCreateInfoKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`library_count`] is the number of pipeline libraries in [`libraries`].
/// - [`libraries`] is a pointer to an array of [`Pipeline`] structures specifying pipeline
///   libraries to use when creating a pipeline.
/// # Description
/// ## Valid Usage
/// - Each element of [`libraries`] **must**  have been created with
///   `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - If [`library_count`] is not `0`, [`libraries`] **must**  be a valid pointer to an array of
///   [`library_count`] valid [`Pipeline`] handles
/// # Related
/// - [`khr_pipeline_library`]
/// - [`Pipeline`]
/// - [`RayTracingPipelineCreateInfoKHR`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`library_count`] is the number of pipeline libraries in
    ///[`libraries`].
    pub library_count: u32,
    ///[`libraries`] is a pointer to an array of [`Pipeline`] structures
    ///specifying pipeline libraries to use when creating a pipeline.
    pub libraries: *const Pipeline,
}
impl<'lt> Default for PipelineLibraryCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            library_count: 0,
            libraries: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineLibraryCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::libraries`]
    pub fn libraries_raw(&self) -> *const Pipeline {
        self.libraries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::libraries`]
    pub fn set_libraries_raw(&mut self, value: *const Pipeline) -> &mut Self {
        self.libraries = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::libraries`]
    pub fn with_libraries_raw(mut self, value: *const Pipeline) -> Self {
        self.libraries = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::library_count`]
    pub fn library_count(&self) -> u32 {
        self.library_count
    }
    ///Gets the value of [`Self::libraries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn libraries(&self) -> &[Pipeline] {
        std::slice::from_raw_parts(self.libraries, self.library_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::library_count`]
    pub fn library_count_mut(&mut self) -> &mut u32 {
        &mut self.library_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::library_count`]
    pub fn set_library_count(&mut self, value: u32) -> &mut Self {
        self.library_count = value;
        self
    }
    ///Sets the value of [`Self::libraries`]
    pub fn set_libraries(&mut self, value: &'lt [crate::vulkan1_0::Pipeline]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.libraries = value.as_ptr();
        self.library_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::library_count`]
    pub fn with_library_count(mut self, value: u32) -> Self {
        self.library_count = value;
        self
    }
    ///Sets the value of [`Self::libraries`]
    pub fn with_libraries(mut self, value: &'lt [crate::vulkan1_0::Pipeline]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.libraries = value.as_ptr();
        self.library_count = len_;
        self
    }
}
