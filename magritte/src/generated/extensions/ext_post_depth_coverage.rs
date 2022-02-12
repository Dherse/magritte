//![VK_EXT_post_depth_coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_post_depth_coverage.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_KHR_post_depth_coverage`which allows the fragment shader to control whether values in the
//![`SampleMask`] built-in input variable reflect the coverage after early
//![depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth) and [stencil](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) tests are applied.This extension adds a new `PostDepthCoverage` execution mode under the
//!`SampleMaskPostDepthCoverage` capability.
//!When this mode is specified along with `EarlyFragmentTests`, the value of
//!an input variable decorated with the
//![[`SampleMask`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-builtin-variables-samplemask) built-in
//!reflects the coverage after the [early fragment
//!tests](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-fragment-earlytest) are applied.
//!Otherwise, it reflects the coverage before the depth and stencil tests.When using GLSL
//! source-based shading languages, the `post_depth_coverage`
//!layout qualifier from GL_ARB_post_depth_coverage or
//!GL_EXT_post_depth_coverage maps to the `PostDepthCoverage` execution
//!mode.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_post_depth_coverage]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_EXT_post_depth_coverage
//!   extension>>)
//!# New constants
//! - [`EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME`]
//! - [`EXT_POST_DEPTH_COVERAGE_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-07-17 (Daniel Koch)
//! - Internal revisions
//!# Other info
//! * 2017-07-17
//!*
//! - This extension requires
//![`SPV_KHR_post_depth_coverage`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_post_depth_coverage.html)
//! - This extension provides API support for
//![`GL_ARB_post_depth_coverage`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_post_depth_coverage.txt)
//!and
//![`GL_EXT_post_depth_coverage`](https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_post_depth_coverage.txt)
//!*
//! - Jeff Bolz, NVIDIA
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
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION")]
pub const EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME")]
pub const EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_post_depth_coverage");
