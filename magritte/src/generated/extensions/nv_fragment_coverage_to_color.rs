//![VK_NV_fragment_coverage_to_color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_coverage_to_color.html) - device extension
//!# Description
//!This extension allows the fragment coverage value, represented as an integer
//!bitmask, to be substituted for a color output being written to a
//!single-component color attachment with integer components (e.g.
//!`VK_FORMAT_R8_UINT`).
//!The functionality provided by this extension is different from simply
//!writing the [`SampleMask`] fragment shader output, in that the coverage
//!value written to the framebuffer is taken after stencil test and depth test,
//!as well as after fragment operations such as alpha-to-coverage.This functionality may be useful
//! for deferred rendering algorithms, where
//!the second pass needs to know which samples belong to which original
//!fragments.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fragment_coverage_to_color]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_fragment_coverage_to_color extension>>)
//!# New structures
//! - Extending [`PipelineMultisampleStateCreateInfo`]:
//! - [`PipelineCoverageToColorStateCreateInfoNV`]
//!# New bitmasks
//! - [`PipelineCoverageToColorStateCreateFlagsNV`]
//!# New constants
//! - [`NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME`]
//! - [`NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
//!# Version History
//! - Revision 1, 2017-05-21 (Jeff Bolz)
//! - Internal revisions
//!# Other info
//! * 2017-05-21
//!*
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`PipelineCoverageToColorStateCreateFlagsNV`]
//! - [`PipelineCoverageToColorStateCreateInfoNV`]
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
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_coverage_to_color");
