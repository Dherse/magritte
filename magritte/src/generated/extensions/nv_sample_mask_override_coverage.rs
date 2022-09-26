//![VK_NV_sample_mask_override_coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_sample_mask_override_coverage.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_NV_sample_mask_override_coverage`
//!The extension provides access to the `OverrideCoverageNV` decoration
//!under the `SampleMaskOverrideCoverageNV` capability.
//!Adding this decoration to a variable with the [`SampleMask`] builtin
//!decoration allows the shader to modify the coverage mask and affect which
//!samples are used to process the fragment.When using GLSL source-based shader languages, the
//! `override_coverage`
//!layout qualifier from `GL_NV_sample_mask_override_coverage` maps to the
//!`OverrideCoverageNV` decoration.
//!To use the `override_coverage` layout qualifier in GLSL the
//!`GL_NV_sample_mask_override_coverage` extension must be enabled.
//!Behavior is described in the `GL_NV_sample_mask_override_coverage` extension
//!spec.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_sample_mask_override_coverage]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_NV_sample_mask_override_coverage extension>>)
//!# New constants
//! - [`NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME`]
//! - [`NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION`]
//!# Version history
//! - Revision 1, 2016-12-08 (Piers Daniell)  - Internal revisions
//!# Other information
//! * 2016-12-08
//! * No known IP claims.
//! * - This extension requires [`SPV_NV_sample_mask_override_coverage`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_sample_mask_override_coverage.html)
//!   - This extension provides API support for [`GL_NV_sample_mask_override_coverage`](https://www.khronos.org/registry/OpenGL/extensions/NV/NV_sample_mask_override_coverage.txt)
//! * - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA
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
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_sample_mask_override_coverage");
