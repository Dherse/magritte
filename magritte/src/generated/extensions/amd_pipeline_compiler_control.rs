use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_AMD_pipeline_compiler_control");
///[VkPipelineCompilerControlCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) - Structure used to pass compilation control flags to a pipeline
///# C Specifications
///The compilation of a pipeline **can** be tuned by adding a
///[`PipelineCompilerControlCreateInfoAMD`] structure to the [`p_next`]
///chain of [`GraphicsPipelineCreateInfo`] or
///[`ComputePipelineCreateInfo`].
///```c
///// Provided by VK_AMD_pipeline_compiler_control
///typedef struct VkPipelineCompilerControlCreateInfoAMD {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkPipelineCompilerControlFlagsAMD    compilerControlFlags;
///} VkPipelineCompilerControlCreateInfoAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`compiler_control_flags`] is a bitmask of [`PipelineCompilerControlFlagBitsAMD`] affecting
///   how the pipeline will be compiled.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
/// - [`compiler_control_flags`]**must** be `0`
///# Related
/// - [`VK_AMD_pipeline_compiler_control`]
/// - [`PipelineCompilerControlFlagsAMD`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`compiler_control_flags`] is a bitmask of
    ///[`PipelineCompilerControlFlagBitsAMD`] affecting how the pipeline
    ///will be compiled.
    compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
impl<'lt> Default for PipelineCompilerControlCreateInfoAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
impl<'lt> PipelineCompilerControlCreateInfoAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::compiler_control_flags`]
    pub fn compiler_control_flags(&self) -> PipelineCompilerControlFlagsAMD {
        self.compiler_control_flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::compiler_control_flags`]
    pub fn compiler_control_flags_mut(&mut self) -> &mut PipelineCompilerControlFlagsAMD {
        &mut self.compiler_control_flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::compiler_control_flags`]
    pub fn set_compiler_control_flags(
        &mut self,
        value: crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
    ) -> &mut Self {
        self.compiler_control_flags = value;
        self
    }
}
