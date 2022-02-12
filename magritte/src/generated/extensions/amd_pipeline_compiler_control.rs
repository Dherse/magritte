//![VK_AMD_pipeline_compiler_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_pipeline_compiler_control.html) - device extension
//!# Description
//!This extension introduces [`PipelineCompilerControlCreateInfoAMD`]
//!structure that can be chained to a pipeline’s creation information to
//!specify additional flags that affect pipeline compilation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_pipeline_compiler_control]
//!   @anteru%0A<<Here describe the issue or question you have about the
//!   VK_AMD_pipeline_compiler_control extension>>)
//!# New structures
//! - Extending [`GraphicsPipelineCreateInfo`], [`ComputePipelineCreateInfo`]:
//! - [`PipelineCompilerControlCreateInfoAMD`]
//!# New enums
//! - [`PipelineCompilerControlFlagBitsAMD`]
//!# New bitmasks
//! - [`PipelineCompilerControlFlagsAMD`]
//!# New constants
//! - [`AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME`]
//! - [`AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2019-07-26 (Tobias Hector)
//! - Initial revision.
//!# Other info
//! * 2019-07-26
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Daniel Rakos, AMD
//! - Maciej Jesionowski, AMD
//! - Tobias Hector, AMD
//!# Related
//! - [`PipelineCompilerControlCreateInfoAMD`]
//! - [`PipelineCompilerControlFlagBitsAMD`]
//! - [`PipelineCompilerControlFlagsAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_AMD_pipeline_compiler_control");
