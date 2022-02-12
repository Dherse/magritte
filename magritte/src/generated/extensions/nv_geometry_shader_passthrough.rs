//![VK_NV_geometry_shader_passthrough](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_geometry_shader_passthrough.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_NV_geometry_shader_passthrough`
//!Geometry shaders provide the ability for applications to process each
//!primitive sent through the graphics pipeline using a programmable shader.
//!However, one common use case treats them largely as a “passthrough”.
//!In this use case, the bulk of the geometry shader code simply copies inputs
//!from each vertex of the input primitive to corresponding outputs in the
//!vertices of the output primitive.
//!Such shaders might also compute values for additional built-in or
//!user-defined per-primitive attributes (e.g., `Layer`) to be assigned to
//!all the vertices of the output primitive.This extension provides access to the `PassthroughNV`
//! decoration under
//!the `GeometryShaderPassthroughNV` capability.
//!Adding this to a geometry shader input variable specifies that the values of
//!this input are copied to the corresponding vertex of the output primitive.When using GLSL
//! source-based shading languages, the `passthrough` layout
//!qualifier from `GL_NV_geometry_shader_passthrough` maps to the
//!`PassthroughNV` decoration.
//!To use the `passthrough` layout, in GLSL the
//!`GL_NV_geometry_shader_passthrough` extension must be enabled.
//!Behaviour is described in the `GL_NV_geometry_shader_passthrough` extension
//!specification.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_geometry_shader_passthrough]
//!   @dgkoch%0A<<Here describe the issue or question you have about the
//!   VK_NV_geometry_shader_passthrough extension>>)
//!# New constants
//! - [`NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME`]
//! - [`NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Should we require or allow a passthrough geometry shader to specify the
//!output layout qualifiers for the output primitive type and maximum vertex
//!count in the SPIR-V?**RESOLVED**: Yes they should be required in the SPIR-V.
//!Per GL_NV_geometry_shader_passthrough they are not permitted in the GLSL
//!source shader, but SPIR-V is lower-level.
//!It is straightforward for the GLSL compiler to infer them from the input
//!primitive type and to explicitly emit them in the SPIR-V according to the
//!following table.2) How does interface matching work with passthrough geometry shaders?**RESOLVED**: This is described in [Passthrough Interface Matching](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry-passthrough-interface).
//!In GL when using passthough geometry shaders in separable mode, all inputs
//!must also be explicitly assigned location layout qualifiers.
//!In Vulkan all SPIR-V shader inputs (except built-ins) must also have
//!location decorations specified.
//!Redeclarations of built-in varables that add the passthrough layout
//!qualifier are exempted from the rule requiring location assignment because
//!built-in variables do not have locations and are matched by `BuiltIn`
//!decoration.
//!# Version History
//! - Revision 1, 2017-02-15 (Daniel Koch)
//! - Internal revisions
//!# Other info
//! * 2017-02-15
//!*
//! - This extension requires
//![`SPV_NV_geometry_shader_passthrough`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_geometry_shader_passthrough.html)
//! - This extension provides API support for
//![`GL_NV_geometry_shader_passthrough`](https://www.khronos.org/registry/OpenGL/extensions/NV/NV_geometry_shader_passthrough.txt)
//! - This extension requires the `geometryShader` feature.
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
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_geometry_shader_passthrough");
