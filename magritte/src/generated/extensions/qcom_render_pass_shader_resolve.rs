//![VK_QCOM_render_pass_shader_resolve](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_shader_resolve.html) - device extension
//!# Description
//!This extension allows a shader resolve to replace fixed-function resolve.Fixed-function resolve
//! is limited in function to simple filters of
//!multisample buffers to a single sample buffer.Fixed-function resolve is more performance
//! efficient and/or power efficient
//!than shader resolve for such simple filters.Shader resolve allows a shader writer to create
//! complex, non-linear
//!filtering of a multisample buffer in the last subpass of a subpass
//!dependency chain.This extension also provides a bit which  **can**  be used to enlarge a sample
//!region dependency to a fragment region dependency, so that a
//!framebuffer-region dependency  **can**  replace a framebuffer-global dependency
//!in some cases.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Bill Licea-Kane [wwlk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_render_pass_shader_resolve]
//!   @wwlk%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_render_pass_shader_resolve extension>>)
//!# New constants
//! - [`QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME`]
//! - [`QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION`]
//! - Extending [`SubpassDescriptionFlagBits`]:  - `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`
//!   - `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`
//!# Known issues & F.A.Q
//!1) Should this extension be named render_pass_shader_resolve? **RESOLVED**  Yes.This is part of
//! suite of small extensions to render pass.Following the style guide, instead of following
//! VK_KHR_create_renderpass2.2) Should the VK_SAMPLE_COUNT_1_BIT be required for each
//! pColorAttachment
//!and the DepthStencilAttachent? **RESOLVED**  No.While this may not be a common use case, and
//! while most fixed-function
//!resolve hardware has this limitation, there is little reason to require a
//!shader resolve to resolve to a single sample buffer.3) Should a shader resolve subpass be the
//! last subpass in a render pass? **RESOLVED**  Yes.To be more specific, it should be the last
//! subpass in a subpass dependency
//!chain.4) Do we need the `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM` bit? **RESOLVED**
//! Yes.This applies when an input attachment’s sample count is equal to
//!`rasterizationSamples`.
//!Further, if `sampleShading` is enabled (explicitly or implicitly) then
//!`minSampleShading` **must**  equal 0.0.However, this bit may be set on any subpass, it is not
//! restricted to a
//!shader resolve subpass.
//!# Version History
//! - Revision 1, 2019-06-28 (wwlk)  - Initial draft
//! - Revision 2, 2019-11-06 (wwlk)  - General clean-up/spec updates  - Added issues
//! - Revision 3, 2019-11-07 (wwlk)  - Typos  - Additional issues  - Clarified that a shader resolve
//!   subpass is the last subpass in a subpass dependency chain
//! - Revision 4, 2020-01-06 (wwlk)  - Change resolution of Issue 1 (*render_pass*, not
//!   *renderpass*)
//!# Other info
//! * 2019-11-07
//! * No known IP claims.
//! * None.
//! * - Srihari Babu Alla, Qualcomm  - Bill Licea-Kane, Qualcomm  - Jeff Leger, Qualcomm
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
#[doc(alias = "VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_QCOM_render_pass_shader_resolve");
