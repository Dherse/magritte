//![VK_KHR_ray_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_query.html) - device extension
//!# Description
//!Rasterization has been the dominant method to produce interactive graphics,
//!but increasing performance of graphics hardware has made ray tracing a
//!viable option for interactive rendering.
//!Being able to integrate ray tracing with traditional rasterization makes it
//!easier for applications to incrementally add ray traced effects to existing
//!applications or to do hybrid approaches with rasterization for primary
//!visibility and ray tracing for secondary queries.Ray queries are available to all shader types,
//! including graphics, compute
//!and ray tracing pipelines.
//!Ray queries are not able to launch additional shaders, instead returning
//!traversal results to the calling shader.This extension adds support for the following SPIR-V
//! extension in Vulkan:
//! - `SPV_KHR_ray_query`
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_KHR_spirv_1_4`]`
//! - Requires `[`VK_KHR_acceleration_structure`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_ray_query]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_ray_query
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceRayQueryFeaturesKHR`]
//!# New constants
//! - [`KHR_RAY_QUERY_EXTENSION_NAME`]
//! - [`KHR_RAY_QUERY_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
//!# Known issues & F.A.Q
//!(1) What are the changes between the public provisional (VK_KHR_ray_tracing
//!v8) release and the final (VK_KHR_acceleration_structure v11 /
//!VK_KHR_ray_query v1) release?
//! - refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation
//!flexibility and decoupling ray query support from ray pipelines:
//! - `[`VK_KHR_acceleration_structure`]` (for acceleration structure
//!operations)
//! - `[`VK_KHR_ray_tracing_pipeline`]` (for ray tracing pipeline and
//!shader stages)
//! - `[`VK_KHR_ray_query`]` (for ray queries in existing shader stages)
//! - Update SPIRV capabilities to use `RayQueryKHR`
//! - extension is no longer provisional
//!# Version History
//! - Revision 1, 2020-11-12 (Mathieu Robart, Daniel Koch, Andrew Garrard)
//! - Decomposition of the specification, from VK_KHR_ray_tracing to
//!VK_KHR_ray_query (#1918,!3912)
//! - update to use `RayQueryKHR` SPIR-V capability
//! - add numerical limits for ray parameters (#2235,!3960)
//! - relax formula for ray intersection candidate determination
//!(#2322,!4080)
//! - restrict traces to TLAS (#2239,!4141)
//! - require `HitT` to be in ray interval for
//!`OpRayQueryGenerateIntersectionKHR` (#2359,!4146)
//! - add ray query shader stages for AS read bit (#2407,!4203)
//!# Other info
//! * 2020-11-12
//!*
//! - This extension requires
//![`SPV_KHR_ray_query`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_ray_query.html)
//! - This extension provides API support for
//![`GLSL_EXT_ray_query`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_ray_query.txt)
//!*
//! - Matthäus Chajdas, AMD
//! - Greg Grebe, AMD
//! - Nicolai Hähnle, AMD
//! - Tobias Hector, AMD
//! - Dave Oldcorn, AMD
//! - Skyler Saleh, AMD
//! - Mathieu Robart, Arm
//! - Marius Bjorge, Arm
//! - Tom Olson, Arm
//! - Sebastian Tafuri, EA
//! - Henrik Rydgard, Embark
//! - Juan Cañada, Epic Games
//! - Patrick Kelly, Epic Games
//! - Yuriy O’Donnell, Epic Games
//! - Michael Doggett, Facebook/Oculus
//! - Andrew Garrard, Imagination
//! - Don Scorgie, Imagination
//! - Dae Kim, Imagination
//! - Joshua Barczak, Intel
//! - Slawek Grajewski, Intel
//! - Jeff Bolz, NVIDIA
//! - Pascal Gautron, NVIDIA
//! - Daniel Koch, NVIDIA
//! - Christoph Kubisch, NVIDIA
//! - Ashwin Lele, NVIDIA
//! - Robert Stepinski, NVIDIA
//! - Martin Stich, NVIDIA
//! - Nuno Subtil, NVIDIA
//! - Eric Werness, NVIDIA
//! - Jon Leech, Khronos
//! - Jeroen van Schijndel, OTOY
//! - Juul Joosten, OTOY
//! - Alex Bourd, Qualcomm
//! - Roman Larionov, Qualcomm
//! - David McAllister, Qualcomm
//! - Spencer Fricke, Samsung
//! - Lewis Gordon, Samsung
//! - Ralph Potter, Samsung
//! - Jasper Bekkers, Traverse Research
//! - Jesse Barker, Unity
//! - Baldur Karlsson, Valve
//!# Related
//! - [`PhysicalDeviceRayQueryFeaturesKHR`]
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
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_query");
