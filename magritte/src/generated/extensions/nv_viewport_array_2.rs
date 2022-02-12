//![VK_NV_viewport_array2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_viewport_array2.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_NV_viewport_array2`
//!which allows a single primitive to be broadcast to multiple viewports and/or
//!multiple layers.
//!A new shader built-in output `ViewportMaskNV` is provided, which allows a
//!single primitive to be output to multiple viewports simultaneously.
//!Also, a new SPIR-V decoration is added to control whether the effective
//!viewport index is added into the variable decorated with the `Layer`
//!built-in decoration.
//!These capabilities allow a single primitive to be output to multiple layers
//!simultaneously.This extension allows variables decorated with the `Layer` and
//!`ViewportIndex` built-ins to be exported from vertex or tessellation
//!shaders, using the `ShaderViewportIndexLayerNV` capability.This extension adds a new
//! `ViewportMaskNV` built-in decoration that is
//!available for output variables in vertex, tessellation evaluation, and
//!geometry shaders, and a new `ViewportRelativeNV` decoration that can be
//!added on variables decorated with `Layer` when using the
//!`ShaderViewportMaskNV` capability.When using GLSL source-based shading languages, the
//! `gl_ViewportMask`[]
//!built-in output variable and `viewport_relative` layout qualifier from
//!`GL_NV_viewport_array2` map to the `ViewportMaskNV` and
//!`ViewportRelativeNV` decorations, respectively.
//!Behaviour is described in the `GL_NV_viewport_array2` extension
//!specificiation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_viewport_array2]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_NV_viewport_array2
//!   extension>>)
//!# New constants
//! - [`NV_VIEWPORT_ARRAY2_EXTENSION_NAME`]
//! - [`NV_VIEWPORT_ARRAY2_SPEC_VERSION`]
//! - [`NV_VIEWPORT_ARRAY_2_EXTENSION_NAME`]
//! - [`NV_VIEWPORT_ARRAY_2_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-02-15 (Daniel Koch)
//! - Internal revisions
//!# Other info
//! * 2017-02-15
//!*
//! - This extension requires
//![`SPV_NV_viewport_array2`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_viewport_array2.html)
//! - This extension provides API support for
//![`GL_NV_viewport_array2`](https://www.khronos.org/registry/OpenGL/extensions/NV/NV_viewport_array2.txt)
//! - This extension requires the `geometryShader` and `multiViewport`
//!features.
//! - This extension interacts with the `tessellationShader` feature.
//!
//!*
//! - Piers Daniell, NVIDIA
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
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION")]
pub const NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME")]
pub const NV_VIEWPORT_ARRAY_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_viewport_array2");
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION")]
pub const NV_VIEWPORT_ARRAY2_SPEC_VERSION: u32 = NV_VIEWPORT_ARRAY_2_SPEC_VERSION;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME")]
pub const NV_VIEWPORT_ARRAY2_EXTENSION_NAME: &'static CStr = NV_VIEWPORT_ARRAY_2_EXTENSION_NAME;
