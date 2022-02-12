//![VK_NV_fragment_shader_barycentric](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shader_barycentric.html) - device extension
//!# Description
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - [`SPV_NV_fragment_shader_barycentric`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_fragment_shader_barycentric.html)The
//!   extension provides access to three additional fragment shader variable
//!decorations in SPIR-V:
//! - `PerVertexNV`, which indicates that a fragment shader input will not
//!have interpolated values, but instead must be accessed with an extra
//!array index that identifies one of the vertices of the primitive
//!producing the fragment
//! - `BaryCoordNV`, which indicates that the variable is a three-component
//!floating-point vector holding barycentric weights for the fragment
//!produced using perspective interpolation
//! - `BaryCoordNoPerspNV`, which indicates that the variable is a
//!three-component floating-point vector holding barycentric weights for
//!the fragment produced using linear interpolationWhen using GLSL source-based shader languages,
//! the following variables from
//!`GL_NV_fragment_shader_barycentric` maps to these SPIR-V built-in
//!decorations:
//! - `in vec3 gl_BaryCoordNV;` → `BaryCoordNV`
//! - `in vec3 gl_BaryCoordNoPerspNV;` → `BaryCoordNoPerspNV`GLSL variables declared using the
//!   `__pervertexNV` GLSL qualifier are
//!expected to be decorated with `PerVertexNV` in SPIR-V.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fragment_shader_barycentric]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the
//!   VK_NV_fragment_shader_barycentric extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`]
//!# New constants
//! - [`NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME`]
//! - [`NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV`
//!# Known issues & F.A.Q
//!(1) The AMD_shader_explicit_vertex_parameter extension provides similar
//!    functionality.
//!    Why write a new extension, and how is this extension different?**RESOLVED**: For the purposes
//! of Vulkan/SPIR-V, we chose to implement a
//!separate extension due to several functional differences.First, the hardware supporting this
//! extension can provide a three-component
//!barycentric weight vector for variables decorated with `BaryCoordNV`,
//!while variables decorated with `BaryCoordSmoothAMD` provide only two
//!components.
//!In some cases, it may be more efficient to explicitly interpolate an
//!attribute via:
//!```c
//!float value = (baryCoordNV.x * v[0].attrib +
//!               baryCoordNV.y * v[1].attrib +
//!               baryCoordNV.z * v[2].attrib);
//!```
//!instead of
//!```c
//!float value = (baryCoordSmoothAMD.x * (v[0].attrib - v[2].attrib) +
//!               baryCoordSmoothAMD.y * (v[1].attrib - v[2].attrib) +
//!               v[2].attrib);
//!```
//!Additionally, the semantics of the decoration `BaryCoordPullModelAMD` do
//!not appear to map to anything supported by the initial hardware
//!implementation of this extension.This extension provides a smaller number of decorations than
//! the AMD
//!extension, as we expect that shaders could derive variables decorated with
//!things like `BaryCoordNoPerspCentroidAMD` with explicit attribute
//!interpolation instructions.
//!One other relevant difference is that explicit per-vertex attribute access
//!using this extension does not require a constant vertex number.(2) Why do the built-in SPIR-V
//! decorations for this extension include two
//!separate built-ins `BaryCoordNV` and `BaryCoordNoPerspNV` when a “no
//!perspective” variable could be decorated with `BaryCoordNV` and
//!`NoPerspective`?**RESOLVED**: The SPIR-V extension for this feature chose to mirror the
//!behavior of the GLSL extension, which provides two built-in variables.
//!Additionally, it is not clear that its a good idea (or even legal) to have
//!two variables using the “same attribute”, but with different interpolation
//!modifiers.
//!# Version History
//! - Revision 1, 2018-08-03 (Pat Brown)
//! - Internal revisions
//!# Other info
//! * 2018-08-03
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_NV_fragment_shader_barycentric`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_fragment_shader_barycentric.html)
//! - This extension provides API support for
//![`GL_NV_fragment_shader_barycentric`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_fragment_shader_barycentric.txt)
//!*
//! - Pat Brown, NVIDIA
//! - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`]
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
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_shader_barycentric");
