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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`compiler_control_flags`] is a bitmask of
    ///[`PipelineCompilerControlFlagBitsAMD`] affecting how the pipeline
    ///will be compiled.
    compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
