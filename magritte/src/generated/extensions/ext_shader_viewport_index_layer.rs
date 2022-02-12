//![VK_EXT_shader_viewport_index_layer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_viewport_index_layer.html) - device extension
//!# Description
//!This extension adds support for the `ShaderViewportIndexLayerEXT`
//!capability from the `SPV_EXT_shader_viewport_index_layer` extension in
//!Vulkan.This extension allows variables decorated with the `Layer` and
//!`ViewportIndex` built-ins to be exported from vertex or tessellation
//!shaders, using the `ShaderViewportIndexLayerEXT` capability.When using GLSL source-based shading
//! languages, the `gl_ViewportIndex`
//!and `gl_Layer` built-in variables map to the SPIR-V `ViewportIndex`
//!and `Layer` built-in decorations, respectively.
//!Behaviour of these variables is extended as described in the
//!`GL_ARB_shader_viewport_layer_array` (or the precursor
//!`GL_AMD_vertex_shader_layer`, `GL_AMD_vertex_shader_viewport_index`, and
//!`GL_NV_viewport_array2` extensions).
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_viewport_index_layer]
//!   @dgkoch%0A<<Here describe the issue or question you have about the
//!   VK_EXT_shader_viewport_index_layer extension>>)
//!# New constants
//! - [`EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME`]
//! - [`EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2017-08-08 (Daniel Koch)
//! - Internal drafts
//!# Other info
//! * 2017-08-08
//!*
//! - Promoted to Vulkan 1.2 Core
//! - This extension requires
//![`SPV_EXT_shader_viewport_index_layer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_viewport_index_layer.html)
//! - This extension provides API support for
//![`GL_ARB_shader_viewport_layer_array`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_viewport_layer_array.txt),
//![`GL_AMD_vertex_shader_layer`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_vertex_shader_layer.txt),
//![`GL_AMD_vertex_shader_viewport_index`](https://www.khronos.org/registry/OpenGL/extensions/AMD/AMD_vertex_shader_viewport_index.txt),
//!and [`GL_NV_viewport_array2`](https://www.khronos.org/registry/OpenGL/extensions/NV/NV_viewport_array2.txt)
//! - This extension requires the `multiViewport` feature.
//! - This extension interacts with the `tessellationShader` feature.
//!*
//! - Piers Daniell, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Jan-Harald Fredriksen, ARM
//! - Daniel Rakos, AMD
//! - Slawomir Grajeswki, Intel
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
#[doc(alias = "VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION")]
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME")]
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_shader_viewport_index_layer");
