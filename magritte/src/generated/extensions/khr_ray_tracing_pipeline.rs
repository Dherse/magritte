//![VK_KHR_ray_tracing_pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html) - device extension
//!# Description
//!Rasterization has been the dominant method to produce interactive graphics,
//!but increasing performance of graphics hardware has made ray tracing a
//!viable option for interactive rendering.
//!Being able to integrate ray tracing with traditional rasterization makes it
//!easier for applications to incrementally add ray traced effects to existing
//!applications or to do hybrid approaches with rasterization for primary
//!visibility and ray tracing for secondary queries.To enable ray tracing, this extension adds a
//! few different categories of new
//!functionality:
//! - A new ray tracing pipeline type with new shader domains: ray generation, intersection,
//!   any-hit, closest hit, miss, and callable
//! - A shader binding indirection table to link shader groups with acceleration structure items
//! - Ray tracing commands which initiate the ray pipeline traversal and invocation of the various
//!   new shader domains depending on which traversal conditions are met
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_KHR_ray_tracing`
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`khr_spirv_1_4`]`
//! - Requires `[`khr_acceleration_structure`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_ray_tracing_pipeline]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_ray_tracing_pipeline
//!   extension>>)
//!# New functions & commands
//! - [`cmd_set_ray_tracing_pipeline_stack_size_khr`]
//! - [`cmd_trace_rays_indirect_khr`]
//! - [`cmd_trace_rays_khr`]
//! - [`create_ray_tracing_pipelines_khr`]
//! - [`get_ray_tracing_capture_replay_shader_group_handles_khr`]
//! - [`get_ray_tracing_shader_group_handles_khr`]
//! - [`get_ray_tracing_shader_group_stack_size_khr`]
//!# New structures
//! - [`RayTracingPipelineCreateInfoKHR`]
//! - [`RayTracingPipelineInterfaceCreateInfoKHR`]
//! - [`RayTracingShaderGroupCreateInfoKHR`]
//! - [`StridedDeviceAddressRegionKHR`]
//! - [`TraceRaysIndirectCommandKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRayTracingPipelineFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//!# New enums
//! - [`RayTracingShaderGroupTypeKHR`]
//! - [`ShaderGroupShaderKHR`]
//!# New constants
//! - [`KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME`]
//! - [`KHR_RAY_TRACING_PIPELINE_SPEC_VERSION`]
//! - [`SHADER_UNUSED_KHR`]
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR`
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`
//! - Extending [`PipelineBindPoint`]:  - `VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR`
//! - Extending [`PipelineCreateFlagBits`]:  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR`
//! - Extending [`ShaderStageFlagBits`]:  - `VK_SHADER_STAGE_ANY_HIT_BIT_KHR`  -
//!   `VK_SHADER_STAGE_CALLABLE_BIT_KHR`  - `VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`  -
//!   `VK_SHADER_STAGE_INTERSECTION_BIT_KHR`  - `VK_SHADER_STAGE_MISS_BIT_KHR`  -
//!   `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!(1) How does this extension differ from VK_NV_ray_tracing? **DISCUSSION** :The following is a
//! summary of the main functional differences between
//!VK_KHR_ray_tracing_pipeline and VK_NV_ray_tracing:
//! - added support for indirect ray tracing ([`cmd_trace_rays_indirect_khr`])
//! - uses SPV_KHR_ray_tracing instead of SPV_NV_ray_tracing  - refer to KHR SPIR-V enums instead of
//!   NV SPIR-V enums (which are functionally equivalent and aliased to the same values).  - added
//!   `RayGeometryIndexKHR` built-in
//! - removed vkCompileDeferredNV compilation functionality and replaced with [deferred host operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations)
//!   interactions for ray tracing
//! - added [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure
//! - extended [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure  - renamed
//!   `maxRecursionDepth` to `maxRayRecursionDepth` and it has a minimum of 1 instead of 31  -
//!   require `shaderGroupHandleSize` to be 32 bytes  - added `maxRayDispatchInvocationCount`,
//!   `shaderGroupHandleAlignment` and `maxRayHitAttributeSize`
//! - reworked geometry structures so they could be better shared between device, host, and indirect
//!   builds
//! - changed SBT parameters to a structure and added size ([`StridedDeviceAddressRegionKHR`])
//! - add parameter for requesting memory requirements for host and/or device build
//! - added [pipeline library](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-library)
//!   support for ray tracing
//! - added [watertightness guarantees](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-watertight)
//! - added no-null-shader pipeline flags
//!   (`VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_*_SHADERS_BIT_KHR`)
//! - added [memory model interactions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-shader-call)
//!   with ray tracing and define how subgroups work and can be repacked
//!(2) Can you give a more detailed comparision of differences and similarities
//!between VK_NV_ray_tracing and VK_KHR_ray_tracing_pipeline? **DISCUSSION** :The following is a
//! more detailed comparision of which commands, structures,
//!and enums are aliased, changed, or removed.
//! - Aliased functionality — enums, structures, and commands that are considered equivalent:  -
//!   [`RayTracingShaderGroupTypeNV`] ↔ [`RayTracingShaderGroupTypeKHR`]  -
//!   [`get_ray_tracing_shader_group_handles_nv`] ↔ [`get_ray_tracing_shader_group_handles_khr`]
//! - Changed enums, structures, and commands:  - [`RayTracingShaderGroupCreateInfoNV`] →
//!   [`RayTracingShaderGroupCreateInfoKHR`] (added `pShaderGroupCaptureReplayHandle`)  -
//!   [`RayTracingPipelineCreateInfoNV`] → [`RayTracingPipelineCreateInfoKHR`] (changed type of
//!   `pGroups`, added `libraries`, `pLibraryInterface`, and `pDynamicState`)  -
//!   [`PhysicalDeviceRayTracingPropertiesNV`] → VkPhysicalDeviceRayTracingPropertiesKHR (renamed
//!   `maxTriangleCount` to `maxPrimitiveCount`, added `shaderGroupHandleCaptureReplaySize`)  -
//!   [`cmd_trace_rays_nv`] → [`cmd_trace_rays_khr`] (params to struct)  -
//!   [`create_ray_tracing_pipelines_nv`] → [`create_ray_tracing_pipelines_khr`] (different struct,
//!   changed functionality)
//! - Added enums, structures and commands:  - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR``VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`, `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR` to [`PipelineCreateFlagBits`]  - [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure  - [`DeviceOrHostAddressKHR`] and [`DeviceOrHostAddressConstKHR`] unions  - [`PipelineLibraryCreateInfoKHR`] struct  - [`RayTracingPipelineInterfaceCreateInfoKHR`] struct  - [`StridedDeviceAddressRegionKHR`] struct  - [`cmd_trace_rays_indirect_khr`] command and [`TraceRaysIndirectCommandKHR`] struct  - [`get_ray_tracing_capture_replay_shader_group_handles_khr`] (shader group capture/replay)  - [`cmd_set_ray_tracing_pipeline_stack_size_khr`] and [`get_ray_tracing_shader_group_stack_size_khr`] commands for stack size control
//! - Functionality removed:  - `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`  - [`compile_deferred_nv`]
//!   command (replaced with `[`khr_deferred_host_operations`]`)
//!(3) What are the changes between the public provisional (VK_KHR_ray_tracing
//!v8) release and the internal provisional (VK_KHR_ray_tracing v9) release?
//! - Require Vulkan 1.1 and SPIR-V 1.4
//! - Added interactions with Vulkan 1.2 and `[`khr_vulkan_memory_model`]`
//! - added creation time capture and replay flags  - added
//!   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR` to
//!   [`PipelineCreateFlagBits`]
//! - replace `VkStridedBufferRegionKHR` with [`StridedDeviceAddressRegionKHR`] and change
//!   [`cmd_trace_rays_khr`], [`cmd_trace_rays_indirect_khr`], to take these for the shader binding
//!   table and use device addresses instead of buffers.
//! - require the shader binding table buffers to have the `VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR` set
//! - make `[`khr_pipeline_library`]` an interaction instead of required extension
//! - rename the `libraries` member of [`RayTracingPipelineCreateInfoKHR`] to `pLibraryInfo` and
//!   make it a pointer
//! - make `[`khr_deferred_host_operations`]` an interaction instead of a required extension (later
//!   went back on this)
//! - added explicit stack size management for ray tracing pipelines  - removed the
//!   `maxCallableSize` member of [`RayTracingPipelineInterfaceCreateInfoKHR`]  - added the
//!   `pDynamicState` member to [`RayTracingPipelineCreateInfoKHR`]  - added
//!   `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` dynamic state for ray tracing pipelines
//!   - added [`get_ray_tracing_shader_group_stack_size_khr`] and
//!   [`cmd_set_ray_tracing_pipeline_stack_size_khr`] commands  - added [`ShaderGroupShaderKHR`]
//!   enum
//! - Added `maxRayDispatchInvocationCount` limit to
//!   [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Added `shaderGroupHandleAlignment` property to
//!   [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Added `maxRayHitAttributeSize` property to [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Clarify deferred host ops for pipeline creation  - [`DeferredOperationKHR`] is now a top-level
//!   parameter for [`create_ray_tracing_pipelines_khr`]  - removed `VkDeferredOperationInfoKHR`
//!   structure  - change deferred host creation/return parameter behavior such that the
//!   implementation can modify such parameters until the deferred host operation completes  -
//!   `[`khr_deferred_host_operations`]` is required again
//!(4) What are the changes between the internal provisional
//!(VK_KHR_ray_tracing v9) release and the final (VK_KHR_acceleration_structure
//!v11 / VK_KHR_ray_tracing_pipeline v1) release?
//! - refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation flexibility and
//!   decoupling ray query support from ray pipelines:  - `[`khr_acceleration_structure`]` (for
//!   acceleration structure operations)  - `[`khr_ray_tracing_pipeline`]` (for ray tracing pipeline
//!   and shader stages)  - `[`khr_ray_query`]` (for ray queries in existing shader stages)
//! - Require `Volatile` for the following builtins in the ray generation, closest hit, miss,
//!   intersection, and callable shader stages:  - `SubgroupSize`, `SubgroupLocalInvocationId`,
//!   `SubgroupEqMask`, `SubgroupGeMask`, `SubgroupGtMask`, `SubgroupLeMask`, `SubgroupLtMask`  -
//!   `SMIDNV`, `WarpIDNV`
//! - clarify buffer usage flags for ray tracing  - `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR`
//!   is added as an alias of `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` and is required on shader binding
//!   table buffers  - `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` is used in
//!   `[`khr_acceleration_structure`]` for `scratchData`
//! - rename `maxRecursionDepth` to `maxRayPipelineRecursionDepth` (pipeline creation) and
//!   `maxRayRecursionDepth` (limit) to reduce confusion
//! - Add queryable `maxRayHitAttributeSize` limit and rename members of
//!   [`RayTracingPipelineInterfaceCreateInfoKHR`] to `maxPipelineRayPayloadSize` and
//!   `maxPipelineRayHitAttributeSize` for clarity
//! - Update SPIRV capabilities to use `RayTracingKHR`
//! - extension is no longer provisional
//! - define synchronization requirements for indirect trace rays and indirect buffer
//!(5) This extension adds gl_InstanceID for the intersection, any-hit, and
//!    closest hit shaders, but in KHR_vulkan_glsl, gl_InstanceID is replaced
//!    with gl_InstanceIndex.
//!    Which should be used for Vulkan in this extension? **RESOLVED** : This extension uses
//! gl_InstanceID and maps it to `InstanceId`
//!in SPIR-V.
//!It is acknowledged that this is different than other shader stages in
//!Vulkan.
//!There are two main reasons for the difference here:
//! - symmetry with gl_PrimitiveID which is also available in these shaders
//! - there is no “baseInstance” relevant for these shaders, and so ID makes it more obvious that
//!   this is zero-based.
//!# Version History
//! - Revision 1, 2020-11-12 (Mathieu Robart, Daniel Koch, Eric Werness, Tobias Hector)  -
//!   Decomposition of the specification, from VK_KHR_ray_tracing to VK_KHR_ray_tracing_pipeline
//!   (#1918,!3912)  - require certain subgroup and sm_shader_builtin shader builtins to be
//!   decorated as volatile in the ray generation, closest hit, miss, intersection, and callable
//!   stages (#1924,!3903,!3954)  - clarify buffer usage flags for ray tracing (#2181,!3939)  -
//!   rename maxRecursionDepth to maxRayPipelineRecursionDepth and maxRayRecursionDepth
//!   (#2203,!3937)  - add queriable maxRayHitAttributeSize and rename members of
//!   VkRayTracingPipelineInterfaceCreateInfoKHR (#2102,!3966)  - update to use `RayTracingKHR`
//!   SPIR-V capability  - add VUs for matching hit group type against geometry type (#2245,!3994)
//!   - require `RayTMaxKHR` be volatile in intersection shaders (#2268,!4030)  - add numerical
//!   limits for ray parameters (#2235,!3960)  - fix SBT indexing rules for device addresses
//!   (#2308,!4079)  - relax formula for ray intersection candidate determination (#2322,!4080)  -
//!   add more details on `ShaderRecordBufferKHR` variables (#2230,!4083)  - clarify valid bits for
//!   `InstanceCustomIndexKHR` (GLSL/GLSL#19,!4128)  - allow at most one `IncomingRayPayloadKHR`,
//!   `IncomingCallableDataKHR`, and `HitAttributeKHR` (!4129)  - add minimum for
//!   maxShaderGroupStride (#2353,!4131)  - require VK_KHR_pipeline_library extension to be
//!   supported (#2348,!4135)  - clarify meaning of 'geometry index' (#2272,!4137)  - restrict
//!   traces to TLAS (#2239,!4141)  - add note about maxPipelineRayPayloadSize (#2383,!4172)  - do
//!   not require raygen shader in pipeline libraries (!4185)  - define sync for indirect trace rays
//!   and indirect buffer (#2407,!4208)
//!# Other info
//! * 2020-11-12
//! * - This extension requires [`SPV_KHR_ray_tracing`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_ray_tracing.html)
//!   - This extension provides API support for [`GLSL_EXT_ray_tracing`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_ray_tracing.txt)
//!   - This extension interacts with [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2)
//!   and `[`khr_vulkan_memory_model`]`, adding the [shader-call-related](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-call-related)
//!   relation of invocations, [shader-call-order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-call-order)
//!   partial order of dynamic instances of instructions, and the [`ShaderCallKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-shadercall)
//!   scope.  - This extension interacts with `[`khr_pipeline_library`]`, enabling pipeline
//!   libraries to be used with ray tracing pipelines and enabling usage of
//!   [`RayTracingPipelineInterfaceCreateInfoKHR`].
//! * - Matthäus Chajdas, AMD  - Greg Grebe, AMD  - Nicolai Hähnle, AMD  - Tobias Hector, AMD  -
//!   Dave Oldcorn, AMD  - Skyler Saleh, AMD  - Mathieu Robart, Arm  - Marius Bjorge, Arm  - Tom
//!   Olson, Arm  - Sebastian Tafuri, EA  - Henrik Rydgard, Embark  - Juan Cañada, Epic Games  -
//!   Patrick Kelly, Epic Games  - Yuriy O’Donnell, Epic Games  - Michael Doggett, Facebook/Oculus
//!   - Andrew Garrard, Imagination  - Don Scorgie, Imagination  - Dae Kim, Imagination  - Joshua
//!   Barczak, Intel  - Slawek Grajewski, Intel  - Jeff Bolz, NVIDIA  - Pascal Gautron, NVIDIA  -
//!   Daniel Koch, NVIDIA  - Christoph Kubisch, NVIDIA  - Ashwin Lele, NVIDIA  - Robert Stepinski,
//!   NVIDIA  - Martin Stich, NVIDIA  - Nuno Subtil, NVIDIA  - Eric Werness, NVIDIA  - Jon Leech,
//!   Khronos  - Jeroen van Schijndel, OTOY  - Juul Joosten, OTOY  - Alex Bourd, Qualcomm  - Roman
//!   Larionov, Qualcomm  - David McAllister, Qualcomm  - Spencer Fricke, Samsung  - Lewis Gordon,
//!   Samsung  - Ralph Potter, Samsung  - Jasper Bekkers, Traverse Research  - Jesse Barker, Unity
//!   - Baldur Karlsson, Valve
//!# Related
//! - [`SHADER_UNUSED_KHR`]
//! - [`PhysicalDeviceRayTracingPipelineFeaturesKHR`]
//! - [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - [`RayTracingPipelineCreateInfoKHR`]
//! - [`RayTracingPipelineInterfaceCreateInfoKHR`]
//! - [`RayTracingShaderGroupCreateInfoKHR`]
//! - [`RayTracingShaderGroupTypeKHR`]
//! - [`ShaderGroupShaderKHR`]
//! - [`StridedDeviceAddressRegionKHR`]
//! - [`TraceRaysIndirectCommandKHR`]
//! - [`cmd_set_ray_tracing_pipeline_stack_size_khr`]
//! - [`cmd_trace_rays_indirect_khr`]
//! - [`cmd_trace_rays_khr`]
//! - [`create_ray_tracing_pipelines_khr`]
//! - [`get_ray_tracing_capture_replay_shader_group_handles_khr`]
//! - [`get_ray_tracing_shader_group_handles_khr`]
//! - [`get_ray_tracing_shader_group_stack_size_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::{
        khr_deferred_host_operations::DeferredOperationKHR, khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    },
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceAddress,
        DeviceSize, Pipeline, PipelineCache, PipelineCreateFlags, PipelineDynamicStateCreateInfo, PipelineLayout,
        PipelineShaderStageCreateInfo, StructureType, VulkanResultCodes,
    },
    vulkan1_3::PipelineCreationFeedbackCreateInfo,
    AsRaw, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    sync::atomic::AtomicBool,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_tracing_pipeline");
///[vkGetRayTracingShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) - Query ray tracing pipeline shader group handles
///# C Specifications
///To query the opaque handles of shaders in the ray tracing pipeline, call:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///VkResult vkGetRayTracingShaderGroupHandlesKHR(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    uint32_t                                    firstGroup,
///    uint32_t                                    groupCount,
///    size_t                                      dataSize,
///    void*                                       pData);
///```
///or the equivalent command
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkGetRayTracingShaderGroupHandlesNV(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    uint32_t                                    firstGroup,
///    uint32_t                                    groupCount,
///    size_t                                      dataSize,
///    void*                                       pData);
///```
///# Parameters
/// - [`device`] is the logical device containing the ray tracing pipeline.
/// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
/// - [`first_group`] is the index of the first group to retrieve a handle     for from the
///   [`RayTracingPipelineCreateInfoKHR::groups`] or [`RayTracingPipelineCreateInfoNV::groups`]
///   array.
/// - [`group_count`] is the number of shader handles to retrieve.
/// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
/// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
///# Description
///## Valid Usage
/// - [`pipeline`] **must**  be a ray tracing pipeline
/// - [`first_group`] **must**  be less than the number of shader groups in [`pipeline`]
/// - The sum of [`first_group`] and [`group_count`] **must**  be less than or equal to the number
///   of shader groups in [`pipeline`]
/// - [`data_size`] **must**  be at least
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_size`] × [`group_count`]
/// - [`pipeline`] **must**  have not been created with `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`] **must**  be greater than `0`
/// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`nv_ray_tracing`]
/// - [`Device`]
/// - [`Pipeline`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
pub type FNGetRayTracingShaderGroupHandlesKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResultCodes,
>;
///[vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) - Query ray tracing capture replay pipeline shader group handles
///# C Specifications
///To query the optional capture handle information of shaders in the ray
///tracing pipeline, call:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///VkResult vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    uint32_t                                    firstGroup,
///    uint32_t                                    groupCount,
///    size_t                                      dataSize,
///    void*                                       pData);
///```
///# Parameters
/// - [`device`] is the logical device containing the ray tracing pipeline.
/// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
/// - [`first_group`] is the index of the first group to retrieve a handle for from the
///   [`RayTracingPipelineCreateInfoKHR::groups`] array.
/// - [`group_count`] is the number of shader handles to retrieve.
/// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
/// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
///# Description
///## Valid Usage
/// - [`pipeline`] **must**  be a ray tracing pipeline
/// - [`first_group`] **must**  be less than the number of shader groups in [`pipeline`]
/// - The sum of [`first_group`] and [`group_count`] **must**  be less than or equal to the number
///   of shader groups in [`pipeline`]
/// - [`data_size`] **must**  be at least
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_capture_replay_size`] ×
///   [`group_count`]
/// -  [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] **must**  be enabled to call this function
/// - [`pipeline`] **must**  have been created with a `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`] **must**  be greater than `0`
/// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`Device`]
/// - [`Pipeline`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
pub type FNGetRayTracingCaptureReplayShaderGroupHandlesKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResultCodes,
>;
///[vkCreateRayTracingPipelinesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) - Creates a new ray tracing pipeline object
///# C Specifications
///To create ray tracing pipelines, call:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///VkResult vkCreateRayTracingPipelinesKHR(
///    VkDevice                                    device,
///    VkDeferredOperationKHR                      deferredOperation,
///    VkPipelineCache                             pipelineCache,
///    uint32_t                                    createInfoCount,
///    const VkRayTracingPipelineCreateInfoKHR*    pCreateInfos,
///    const VkAllocationCallbacks*                pAllocator,
///    VkPipeline*                                 pPipelines);
///```
///# Parameters
/// - [`device`] is the logical device that creates the ray tracing pipelines.
/// - [`deferred_operation`] is [`crate::Handle::null`] or the handle of a valid [`DeferredOperationKHR`][request deferral](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations-requesting) object for this command.
/// - [`pipeline_cache`] is either [`crate::Handle::null`], indicating that pipeline caching is disabled, or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache) object, in which case use of that cache is enabled for the duration of the command.
/// - [`create_info_count`] is the length of the [`p_create_infos`] and [`p_pipelines`] arrays.
/// - [`p_create_infos`] is a pointer to an array of [`RayTracingPipelineCreateInfoKHR`] structures.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_pipelines`] is a pointer to an array in which the resulting ray tracing pipeline objects
///   are returned.
///# Description
///The `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` error is returned if the
///implementation is unable to re-use the shader group handles provided in
///[`RayTracingShaderGroupCreateInfoKHR::shader_group_capture_replay_handle`]
///when
///[`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`]
///is enabled.
///## Valid Usage
/// - If the `flags` member of any element of [`p_create_infos`] contains the
///   `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and the `basePipelineIndex` member of that same
///   element is not `-1`, `basePipelineIndex` **must**  be less than the index into
///   [`p_create_infos`] that corresponds to that element
/// - If the `flags` member of any element of [`p_create_infos`] contains the
///   `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, the base pipeline  **must**  have been created with
///   the `VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT` flag set
/// - `flags` **must**  not contain the `VK_PIPELINE_CREATE_DISPATCH_BASE` flag
/// - If [`pipeline_cache`] was created with `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`,
///   host access to [`pipeline_cache`] **must**  be [externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#fundamentals-threadingbehavior)
///
/// - If [`deferred_operation`] is not [`crate::Handle::null`], it  **must**  be a valid
///   [`DeferredOperationKHR`] object
/// - Any previous deferred operation that was associated with [`deferred_operation`] **must**  be
///   complete
/// - The [`rayTracingPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipeline)
///   feature  **must**  be enabled
/// - If [`deferred_operation`] is not [`crate::Handle::null`], the `flags` member of elements of
///   [`p_create_infos`] **must**  not include `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`deferred_operation`] is not [`crate::Handle::null`], [`deferred_operation`] **must**  be
///   a valid [`DeferredOperationKHR`] handle
/// - If [`pipeline_cache`] is not [`crate::Handle::null`], [`pipeline_cache`] **must**  be a valid
///   [`PipelineCache`] handle
/// - [`p_create_infos`] **must**  be a valid pointer to an array of [`create_info_count`] valid
///   [`RayTracingPipelineCreateInfoKHR`] structures
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_pipelines`] **must**  be a valid pointer to an array of [`create_info_count`][`Pipeline`]
///   handles
/// - [`create_info_count`] **must**  be greater than `0`
/// - If [`deferred_operation`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`device`]
/// - If [`pipeline_cache`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_OPERATION_DEFERRED_KHR`  - `VK_OPERATION_NOT_DEFERRED_KHR`  -
///   `VK_PIPELINE_COMPILE_REQUIRED_EXT`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`AllocationCallbacks`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
/// - [`Pipeline`]
/// - [`PipelineCache`]
/// - [`RayTracingPipelineCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
pub type FNCreateRayTracingPipelinesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_pipelines: *mut Pipeline,
    ) -> VulkanResultCodes,
>;
///[vkGetRayTracingShaderGroupStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) - Query ray tracing pipeline shader group shader stack size
///# C Specifications
///To query the pipeline stack size of shaders in a shader group in the ray
///tracing pipeline, call:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///VkDeviceSize vkGetRayTracingShaderGroupStackSizeKHR(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    uint32_t                                    group,
///    VkShaderGroupShaderKHR                      groupShader);
///```
///# Parameters
/// - [`device`] is the logical device containing the ray tracing pipeline.
/// - [`pipeline`] is the ray tracing pipeline object containing the shaders groups.
/// - [`group`] is the index of the shader group to query.
/// - [`group_shader`] is the type of shader from the group to query.
///# Description
///The return value is the ray tracing pipeline stack size in bytes for the
///specified shader as called from the specified shader group.
///## Valid Usage
/// - [`pipeline`] **must**  be a ray tracing pipeline
/// - The value of [`group`] must be less than the number of shader groups in [`pipeline`]
/// - The shader identified by [`group_shader`] in [`group`] **must**  not be [`SHADER_UNUSED_KHR`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`group_shader`] **must**  be a valid [`ShaderGroupShaderKHR`] value
/// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`Device`]
/// - [`Pipeline`]
/// - [`ShaderGroupShaderKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
pub type FNGetRayTracingShaderGroupStackSizeKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize,
>;
///[vkCmdTraceRaysKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html) - Initialize a ray tracing dispatch
///# C Specifications
///To dispatch ray tracing use:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///void vkCmdTraceRaysKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkStridedDeviceAddressRegionKHR*      pRaygenShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pMissShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pHitShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pCallableShaderBindingTable,
///    uint32_t                                    width,
///    uint32_t                                    height,
///    uint32_t                                    depth);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`p_raygen_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the ray generation shader stage.
/// - [`p_miss_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the miss shader stage.
/// - [`p_hit_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the hit shader stage.
/// - [`p_callable_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
///   shader binding table data for the callable shader stage.
/// - [`width`] is the width of the ray trace query dimensions.
/// - [`height`] is height of the ray trace query dimensions.
/// - [`depth`] is depth of the ray trace query dimensions.
///# Description
///When the command is executed, a ray generation group of [`width`]
///× [`height`] × [`depth`] rays is assembled.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so
///   after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - Any shader group handle referenced by this call  **must**  have been queried from the
///   currently bound ray tracing pipeline
///
/// - This command  **must**  not cause a shader call instruction to be executed from a shader
///   invocation with a [recursion depth]() greater than the value of `maxPipelineRayRecursionDepth`
///   used to create the bound ray tracing pipeline
/// - If the buffer from which `pRayGenShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pRayGenShaderBindingTable->deviceAddress` is queried  **must**
///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pRayGenShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `size` member of `pRayGenShaderBindingTable` **must**  be equal to its `stride` member
/// - If the buffer from which `pMissShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pMissShaderBindingTable->deviceAddress` is queried  **must**  have
///   been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pMissShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the buffer from which `pHitShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pHitShaderBindingTable->deviceAddress` is queried  **must**  have
///   been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pHitShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the buffer from which `pCallableShaderBindingTable->deviceAddress` was queried is
///   non-sparse then it  **must**  be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// - The buffer from which the `pCallableShaderBindingTable->deviceAddress` is queried  **must**
///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pCallableShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, the `deviceAddress`
///   member of [`p_hit_shader_binding_table`] **must**  not be zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, the `deviceAddress`
///   member of [`p_hit_shader_binding_table`] **must**  not be zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, the shader group handle
///   identified by [`p_miss_shader_binding_table`] **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
///   any-hit shader  **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute a
///   closest hit shader  **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
///   intersection shader  **must**  not be set to zero
/// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this call
///   from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR` **must**  have been
///   created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR`
/// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this call
///   from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR` **must**  have been
///   created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`
///
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - [`width`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][0]
/// - [`height`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][1]
/// - [`depth`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][2]
/// - [`width`] × [`height`] × [`depth`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_dispatch_invocation_count`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_raygen_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_miss_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_hit_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_callable_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`CommandBuffer`]
/// - [`StridedDeviceAddressRegionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdTraceRaysKHR")]
pub type FNCmdTraceRaysKhr = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ),
>;
///[vkCmdTraceRaysIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) - Initialize an indirect ray tracing dispatch
///# C Specifications
///To dispatch ray tracing, with some parameters sourced on the device, use:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///void vkCmdTraceRaysIndirectKHR(
///    VkCommandBuffer                             commandBuffer,
///    const VkStridedDeviceAddressRegionKHR*      pRaygenShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pMissShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pHitShaderBindingTable,
///    const VkStridedDeviceAddressRegionKHR*      pCallableShaderBindingTable,
///    VkDeviceAddress                             indirectDeviceAddress);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`p_raygen_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the ray generation shader stage.
/// - [`p_miss_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the miss shader stage.
/// - [`p_hit_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the shader
///   binding table data for the hit shader stage.
/// - [`p_callable_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
///   shader binding table data for the callable shader stage.
/// - [`indirect_device_address`] is a buffer device address which is a pointer to a
///   [`TraceRaysIndirectCommandKHR`] structure containing the trace ray parameters.
///# Description
///[`cmd_trace_rays_indirect_khr`] behaves similarly to [`cmd_trace_rays_khr`]
///except that the ray trace query dimensions are read by the device from
///[`indirect_device_address`] during execution.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so
///   after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - Any shader group handle referenced by this call  **must**  have been queried from the
///   currently bound ray tracing pipeline
///
/// - This command  **must**  not cause a shader call instruction to be executed from a shader
///   invocation with a [recursion depth]() greater than the value of `maxPipelineRayRecursionDepth`
///   used to create the bound ray tracing pipeline
/// - If the buffer from which `pRayGenShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pRayGenShaderBindingTable->deviceAddress` is queried  **must**
///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pRayGenShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `size` member of `pRayGenShaderBindingTable` **must**  be equal to its `stride` member
/// - If the buffer from which `pMissShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pMissShaderBindingTable->deviceAddress` is queried  **must**  have
///   been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pMissShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the buffer from which `pHitShaderBindingTable->deviceAddress` was queried is non-sparse
///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which the `pHitShaderBindingTable->deviceAddress` is queried  **must**  have
///   been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pHitShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the buffer from which `pCallableShaderBindingTable->deviceAddress` was queried is
///   non-sparse then it  **must**  be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// - The buffer from which the `pCallableShaderBindingTable->deviceAddress` is queried  **must**
///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
/// - `pCallableShaderBindingTable->deviceAddress` **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
/// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
/// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, the `deviceAddress`
///   member of [`p_hit_shader_binding_table`] **must**  not be zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, the `deviceAddress`
///   member of [`p_hit_shader_binding_table`] **must**  not be zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, the shader group handle
///   identified by [`p_miss_shader_binding_table`] **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
///   any-hit shader  **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute a
///   closest hit shader  **must**  not be set to zero
/// - If the currently bound ray tracing pipeline was created with `flags` that included
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, entries in
///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
///   intersection shader  **must**  not be set to zero
/// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this call
///   from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR` **must**  have been
///   created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR`
/// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this call
///   from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR` **must**  have been
///   created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`
///
/// - If the buffer from which [`indirect_device_address`] was queried is non-sparse then it
///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - The buffer from which [`indirect_device_address`] was queried  **must**  have been created
///   with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - [`indirect_device_address`] **must**  be a multiple of `4`
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - All device addresses between [`indirect_device_address`] and [`indirect_device_address`] +
///   `sizeof`([`TraceRaysIndirectCommandKHR`]) - 1 **must**  be in the buffer device address range
///   of the same buffer
/// - The [[`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_trace_rays_indirect`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipelineTraceRaysIndirect)
///   feature  **must**  be enabled
/// -    If the bound ray tracing pipeline was created with `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`[`PhysicalDeviceRayTracingMotionBlurFeaturesNV::ray_tracing_motion_blur_pipeline_trace_rays_indirect`] feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_raygen_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_miss_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_hit_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`p_callable_shader_binding_table`] **must**  be a valid pointer to a valid
///   [`StridedDeviceAddressRegionKHR`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`CommandBuffer`]
/// - [`DeviceAddress`]
/// - [`StridedDeviceAddressRegionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
pub type FNCmdTraceRaysIndirectKhr = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ),
>;
///[vkCmdSetRayTracingPipelineStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) - Set the stack size dynamically for a ray tracing pipeline
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the stack size for a ray
///tracing pipeline, call:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///void vkCmdSetRayTracingPipelineStackSizeKHR(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    pipelineStackSize);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`pipeline_stack_size`] is the stack size to use for subsequent ray tracing trace commands.
///# Description
///This command sets the stack size for subsequent ray tracing commands when
///the ray tracing pipeline is created with
///`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, the stack size is computed as described in
///[Ray Tracing Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).
///## Valid Usage
/// - [`pipeline_stack_size`] **must**  be large enough for any dynamic execution through the
///   shaders in the ray tracing pipeline used by a subsequent trace call
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`CommandBuffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
pub type FNCmdSetRayTracingPipelineStackSizeKhr =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32)>;
///[VkRayTracingShaderGroupTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) - Shader group types
///# C Specifications
///Possible values of `type` in [`RayTracingShaderGroupCreateInfoKHR`]
///are:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef enum VkRayTracingShaderGroupTypeKHR {
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV = VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV =
/// VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV =
/// VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR,
///} VkRayTracingShaderGroupTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkRayTracingShaderGroupTypeKHR VkRayTracingShaderGroupTypeNV;
///```
///# Description
/// - [`GENERAL`] indicates a shader group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
///   `VK_SHADER_STAGE_MISS_BIT_KHR`, or `VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
/// - [`TRIANGLES_HIT_GROUP`] specifies a shader group that only hits triangles and  **must**  not
///   contain an intersection shader, only closest hit and any-hit shaders.
/// - [`PROCEDURAL_HIT_GROUP`] specifies a shader group that only intersects with custom geometry
///   and  **must**  contain an intersection shader and  **may**  contain closest hit and any-hit
///   shaders.
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`nv_ray_tracing`]
/// - [`RayTracingShaderGroupCreateInfoKHR`]
/// - [`RayTracingShaderGroupCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl const Default for RayTracingShaderGroupTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl RayTracingShaderGroupTypeKHR {
    ///[`GENERAL`] indicates a shader
    ///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
    ///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
    ///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
    pub const GENERAL: Self = Self(0);
    ///[`TRIANGLES_HIT_GROUP`] specifies
    ///a shader group that only hits triangles and  **must**  not contain an
    ///intersection shader, only closest hit and any-hit shaders.
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    ///[`PROCEDURAL_HIT_GROUP`]
    ///specifies a shader group that only intersects with custom geometry and
    /// **must**  contain an intersection shader and  **may**  contain closest hit and
    ///any-hit shaders.
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(RayTracingShaderGroupTypeKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == RayTracingShaderGroupTypeKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        RayTracingShaderGroupTypeKHR::GENERAL => f.write_str("GENERAL")?,
                        RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP => f.write_str("TRIANGLES_HIT_GROUP")?,
                        RayTracingShaderGroupTypeKHR::PROCEDURAL_HIT_GROUP => f.write_str("PROCEDURAL_HIT_GROUP")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(RayTracingShaderGroupTypeKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkShaderGroupShaderKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html) - Shader group shaders
///# C Specifications
///Possible values of `groupShader` in
///[`get_ray_tracing_shader_group_stack_size_khr`] are:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef enum VkShaderGroupShaderKHR {
///    VK_SHADER_GROUP_SHADER_GENERAL_KHR = 0,
///    VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR = 1,
///    VK_SHADER_GROUP_SHADER_ANY_HIT_KHR = 2,
///    VK_SHADER_GROUP_SHADER_INTERSECTION_KHR = 3,
///} VkShaderGroupShaderKHR;
///```
///# Description
/// - [`GENERAL`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::general_shader`]
/// - [`CLOSEST_HIT`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::closest_hit_shader`]
/// - [`ANY_HIT`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::any_hit_shader`]
/// - [`INTERSECTION`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::intersection_shader`]
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`get_ray_tracing_shader_group_stack_size_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderGroupShaderKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ShaderGroupShaderKHR(i32);
impl const Default for ShaderGroupShaderKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl ShaderGroupShaderKHR {
    ///[`GENERAL`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`generalShader`
    pub const GENERAL: Self = Self(0);
    ///[`CLOSEST_HIT`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`closestHitShader`
    pub const CLOSEST_HIT: Self = Self(1);
    ///[`ANY_HIT`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`anyHitShader`
    pub const ANY_HIT: Self = Self(2);
    ///[`INTERSECTION`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`intersectionShader`
    pub const INTERSECTION: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ShaderGroupShaderKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ShaderGroupShaderKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        ShaderGroupShaderKHR::GENERAL => f.write_str("GENERAL")?,
                        ShaderGroupShaderKHR::CLOSEST_HIT => f.write_str("CLOSEST_HIT")?,
                        ShaderGroupShaderKHR::ANY_HIT => f.write_str("ANY_HIT")?,
                        ShaderGroupShaderKHR::INTERSECTION => f.write_str("INTERSECTION")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ShaderGroupShaderKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkRayTracingShaderGroupCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) - Structure specifying shaders in a shader group
///# C Specifications
///The [`RayTracingShaderGroupCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkRayTracingShaderGroupCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkRayTracingShaderGroupTypeKHR    type;
///    uint32_t                          generalShader;
///    uint32_t                          closestHitShader;
///    uint32_t                          anyHitShader;
///    uint32_t                          intersectionShader;
///    const void*                       pShaderGroupCaptureReplayHandle;
///} VkRayTracingShaderGroupCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is the type of hit group specified in this structure.
/// - [`general_shader`] is the index of the ray generation, miss, or callable shader from
///   [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
/// - [`closest_hit_shader`] is the optional index of the closest hit shader from
///   [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`any_hit_shader`] is the optional index of the any-hit shader from
///   [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`intersection_shader`] is the index of the intersection shader from
///   [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`shader_group_capture_replay_handle`] is `NULL` or a pointer to replay information for this
///   shader group. Ignored if
///   [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`]
///   is [`FALSE`].
///# Description
///## Valid Usage
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`general_shader`]
///   **must**  be a valid index into [`RayTracingPipelineCreateInfoKHR::stages`] referring to a
///   shader of `VK_SHADER_STAGE_RAYGEN_BIT_KHR`, `VK_SHADER_STAGE_MISS_BIT_KHR`, or
///   `VK_SHADER_STAGE_CALLABLE_BIT_KHR`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`closest_hit_shader`],
///   [`any_hit_shader`], and [`intersection_shader`] **must**  be [`SHADER_UNUSED_KHR`]
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR` then
///   [`intersection_shader`] **must**  be a valid index into
///   [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of
///   `VK_SHADER_STAGE_INTERSECTION_BIT_KHR`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` then
///   [`intersection_shader`] **must**  be [`SHADER_UNUSED_KHR`]
/// - [`closest_hit_shader`] **must**  be either [`SHADER_UNUSED_KHR`] or a valid index into
///   [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of
///   `VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`
/// - [`any_hit_shader`] **must**  be either [`SHADER_UNUSED_KHR`] or a valid index into
///   [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of
///   `VK_SHADER_STAGE_ANY_HIT_BIT_KHR`
/// -    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then [`shader_group_capture_replay_handle`] **must**  not be provided if it has not been provided on a previous call to ray tracing pipeline creation
/// -    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then the caller  **must**  guarantee that no ray tracing pipeline creation commands with [`shader_group_capture_replay_handle`] provided execute simultaneously with ray tracing pipeline creation commands without [`shader_group_capture_replay_handle`] provided
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`RayTracingShaderGroupTypeKHR`] value
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`RayTracingPipelineCreateInfoKHR`]
/// - [`RayTracingShaderGroupTypeKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is the type of hit group specified in this structure.
    pub type_: RayTracingShaderGroupTypeKHR,
    ///[`general_shader`] is the index of the ray generation, miss, or
    ///callable shader from
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    pub general_shader: u32,
    ///[`closest_hit_shader`] is the optional index of the closest hit shader
    ///from [`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group
    ///if the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    pub closest_hit_shader: u32,
    ///[`any_hit_shader`] is the optional index of the any-hit shader from
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    pub any_hit_shader: u32,
    ///[`intersection_shader`] is the index of the intersection shader from
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    pub intersection_shader: u32,
    ///[`shader_group_capture_replay_handle`] is `NULL` or a pointer to replay
    ///information for this shader group.
    ///Ignored if
    ///[`PhysicalDeviceRayTracingPipelineFeaturesKHR`]::`rayTracingPipelineShaderGroupHandleCaptureReplay`
    ///is [`FALSE`].
    pub shader_group_capture_replay_handle: *const c_void,
}
impl<'lt> Default for RayTracingShaderGroupCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            type_: Default::default(),
            general_shader: 0,
            closest_hit_shader: 0,
            any_hit_shader: 0,
            intersection_shader: 0,
            shader_group_capture_replay_handle: std::ptr::null(),
        }
    }
}
impl<'lt> RayTracingShaderGroupCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_group_capture_replay_handle`]
    pub fn shader_group_capture_replay_handle_raw(&self) -> *const c_void {
        self.shader_group_capture_replay_handle
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_group_capture_replay_handle`]
    pub fn set_shader_group_capture_replay_handle_raw(&mut self, value: *const c_void) -> &mut Self {
        self.shader_group_capture_replay_handle = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_group_capture_replay_handle`]
    pub fn with_shader_group_capture_replay_handle_raw(mut self, value: *const c_void) -> Self {
        self.shader_group_capture_replay_handle = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> RayTracingShaderGroupTypeKHR {
        self.type_
    }
    ///Gets the value of [`Self::general_shader`]
    pub fn general_shader(&self) -> u32 {
        self.general_shader
    }
    ///Gets the value of [`Self::closest_hit_shader`]
    pub fn closest_hit_shader(&self) -> u32 {
        self.closest_hit_shader
    }
    ///Gets the value of [`Self::any_hit_shader`]
    pub fn any_hit_shader(&self) -> u32 {
        self.any_hit_shader
    }
    ///Gets the value of [`Self::intersection_shader`]
    pub fn intersection_shader(&self) -> u32 {
        self.intersection_shader
    }
    ///Gets the value of [`Self::shader_group_capture_replay_handle`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn shader_group_capture_replay_handle(&self) -> &c_void {
        &*self.shader_group_capture_replay_handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut RayTracingShaderGroupTypeKHR {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::general_shader`]
    pub fn general_shader_mut(&mut self) -> &mut u32 {
        &mut self.general_shader
    }
    ///Gets a mutable reference to the value of [`Self::closest_hit_shader`]
    pub fn closest_hit_shader_mut(&mut self) -> &mut u32 {
        &mut self.closest_hit_shader
    }
    ///Gets a mutable reference to the value of [`Self::any_hit_shader`]
    pub fn any_hit_shader_mut(&mut self) -> &mut u32 {
        &mut self.any_hit_shader
    }
    ///Gets a mutable reference to the value of [`Self::intersection_shader`]
    pub fn intersection_shader_mut(&mut self) -> &mut u32 {
        &mut self.intersection_shader
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::general_shader`]
    pub fn set_general_shader(&mut self, value: u32) -> &mut Self {
        self.general_shader = value;
        self
    }
    ///Sets the value of [`Self::closest_hit_shader`]
    pub fn set_closest_hit_shader(&mut self, value: u32) -> &mut Self {
        self.closest_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::any_hit_shader`]
    pub fn set_any_hit_shader(&mut self, value: u32) -> &mut Self {
        self.any_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::intersection_shader`]
    pub fn set_intersection_shader(&mut self, value: u32) -> &mut Self {
        self.intersection_shader = value;
        self
    }
    ///Sets the value of [`Self::shader_group_capture_replay_handle`]
    pub fn set_shader_group_capture_replay_handle(&mut self, value: &'lt std::ffi::c_void) -> &mut Self {
        self.shader_group_capture_replay_handle = value as *const _;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::type_`]
    pub fn with_type_(
        mut self,
        value: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    ) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::general_shader`]
    pub fn with_general_shader(mut self, value: u32) -> Self {
        self.general_shader = value;
        self
    }
    ///Sets the value of [`Self::closest_hit_shader`]
    pub fn with_closest_hit_shader(mut self, value: u32) -> Self {
        self.closest_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::any_hit_shader`]
    pub fn with_any_hit_shader(mut self, value: u32) -> Self {
        self.any_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::intersection_shader`]
    pub fn with_intersection_shader(mut self, value: u32) -> Self {
        self.intersection_shader = value;
        self
    }
    ///Sets the value of [`Self::shader_group_capture_replay_handle`]
    pub fn with_shader_group_capture_replay_handle(mut self, value: &'lt std::ffi::c_void) -> Self {
        self.shader_group_capture_replay_handle = value as *const _;
        self
    }
}
///[VkRayTracingPipelineCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html) - Structure specifying parameters of a newly created ray tracing pipeline
///# C Specifications
///The [`RayTracingPipelineCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkRayTracingPipelineCreateInfoKHR {
///    VkStructureType                                      sType;
///    const void*                                          pNext;
///    VkPipelineCreateFlags                                flags;
///    uint32_t                                             stageCount;
///    const VkPipelineShaderStageCreateInfo*               pStages;
///    uint32_t                                             groupCount;
///    const VkRayTracingShaderGroupCreateInfoKHR*          pGroups;
///    uint32_t                                             maxPipelineRayRecursionDepth;
///    const VkPipelineLibraryCreateInfoKHR*                pLibraryInfo;
///    const VkRayTracingPipelineInterfaceCreateInfoKHR*    pLibraryInterface;
///    const VkPipelineDynamicStateCreateInfo*              pDynamicState;
///    VkPipelineLayout                                     layout;
///    VkPipeline                                           basePipelineHandle;
///    int32_t                                              basePipelineIndex;
///} VkRayTracingPipelineCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying how the pipeline will be
///   generated.
/// - [`stage_count`] is the number of entries in the [`stages`] array.
/// - [`stages`] is a pointer to an array of [`stage_count`][`PipelineShaderStageCreateInfo`]
///   structures describing the set of the shader stages to be included in the ray tracing pipeline.
/// - [`group_count`] is the number of entries in the [`groups`] array.
/// - [`groups`] is a pointer to an array of [`group_count`][`RayTracingShaderGroupCreateInfoKHR`]
///   structures describing the set of the shader stages to be included in each shader group in the
///   ray tracing pipeline.
/// - [`max_pipeline_ray_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth)
///   of shaders executed by this pipeline.
/// - [`library_info`] is a pointer to a [`PipelineLibraryCreateInfoKHR`] structure defining
///   pipeline libraries to include.
/// - [`library_interface`] is a pointer to a [`RayTracingPipelineInterfaceCreateInfoKHR`] structure
///   defining additional information when using pipeline libraries.
/// - [`dynamic_state`] is a pointer to a [`PipelineDynamicStateCreateInfo`] structure, and is used
///   to indicate which properties of the pipeline state object are dynamic and  **can**  be changed
///   independently of the pipeline state. This  **can**  be `NULL`, which means no state in the
///   pipeline is considered dynamic.
/// - [`layout`] is the description of binding locations used by both the pipeline and descriptor
///   sets used with the pipeline.
/// - [`base_pipeline_handle`] is a pipeline to derive from.
/// - [`base_pipeline_index`] is an index into the `pCreateInfos` parameter to use as a pipeline to
///   derive from.
///# Description
///The parameters [`base_pipeline_handle`] and [`base_pipeline_index`] are
///described in more detail in [Pipeline
///Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).When `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR` is specified, this pipeline
///defines a *pipeline library* which  **cannot**  be bound as a ray tracing
///pipeline directly.
///Instead, pipeline libraries define common shaders and shader groups which
/// **can**  be included in future pipeline creation.If pipeline libraries are included in
/// [`library_info`], shaders defined in
///those libraries are treated as if they were defined as additional entries in
///[`stages`], appended in the order they appear in the `pLibraries`
///array and in the [`stages`] array when those libraries were defined.When referencing shader
/// groups in order to obtain a shader group handle,
///groups defined in those libraries are treated as if they were defined as
///additional entries in [`groups`], appended in the order they appear in
///the `pLibraries` array and in the [`groups`] array when those
///libraries were defined.
///The shaders these groups reference are set when the pipeline library is
///created, referencing those specified in the pipeline library, not in the
///pipeline that includes it.The default stack size for a pipeline if
///`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` is not provided
///is computed as described in [Ray Tracing
///Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).
///## Valid Usage
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_index`] is `-1`, [`base_pipeline_handle`] **must**  be a valid handle to a ray
///   tracing [`Pipeline`]
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_handle`] is [`crate::Handle::null`], [`base_pipeline_index`] **must**  be a
///   valid index into the calling command’s `pCreateInfos` parameter
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_index`] is not `-1`, [`base_pipeline_handle`] **must**  be
///   [`crate::Handle::null`]
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_handle`] is not [`crate::Handle::null`], [`base_pipeline_index`] **must**  be
///   `-1`
/// -    The shader code for the entry points identified by [`stages`], and the rest of the state identified by this structure  **must**  adhere to the pipeline linking rules described in the [Shader Interfaces](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#interfaces) chapter
/// - [`layout`] **must**  be [consistent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#descriptorsets-pipelinelayout-consistency)
///   with all shaders specified in [`stages`]
/// - The number of resources in [`layout`] accessible to each shader stage that is used by the
///   pipeline  **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_resources`]
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
/// - If the [`pipelineCreationCacheControl`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-pipelineCreationCacheControl)
///   feature is not enabled, [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` or
///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
/// - If [`flags`] does not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, the `stage` member of at
///   least one element of [`stages`], including those implicitly added by [`library_info`],
///   **must**  be `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
/// - [`max_pipeline_ray_recursion_depth`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_recursion_depth`]
/// - If [`flags`] includes `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, [`library_interface`] **must**
///   not be `NULL`
/// - If [`library_info`] is not `NULL` and its `libraryCount` member is greater than `0`, its
///   [`library_interface`] member  **must**  not be `NULL`
/// - Each element of `pLibraryInfo->pLibraries` **must**  have been created with the value of
///   [`max_pipeline_ray_recursion_depth`] equal to that in this pipeline
/// - If [`library_info`] is not `NULL`, each element of its `pLibraries` member  **must**  have
///   been created with a [`layout`] that is compatible with the [`layout`] in this pipeline
/// - If [`library_info`] is not `NULL`, each element of its `pLibraries` member  **must**  have
///   been created with values of the `maxPipelineRayPayloadSize` and
///   `maxPipelineRayHitAttributeSize` members of [`library_interface`] equal to those in this
///   pipeline
/// - If [`flags`] includes
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, each
///   element of `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`,
///   each element of `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`,
///   each element of `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, each
///   element of `pLibraryInfo->pLibraries` **must**  have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR` bit set
/// - If the `[`khr_pipeline_library`]` extension is not enabled, [`library_info`] and
///   [`library_interface`] **must**  be `NULL`
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, for
///   any element of [`groups`] with a `type` of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `anyHitShader` of that
///   element  **must**  not be [`SHADER_UNUSED_KHR`]
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`,
///   for any element of [`groups`] with a `type` of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `closestHitShader` of that
///   element  **must**  not be [`SHADER_UNUSED_KHR`]
/// - If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling)
///   feature is not enabled, [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// - If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling)
///   feature is not enabled, [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
/// - [`flags`] **must**  not include both `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
///   and `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// -    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, [`rayTracingPipelineShaderGroupHandleCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipelineShaderGroupHandleCaptureReplay) **must**  be enabled
/// -    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] is [`TRUE`] and the `pShaderGroupCaptureReplayHandle` member of any element of [`groups`] is not `NULL`, [`flags`] **must**  include `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
/// - If [`library_info`] is not `NULL` and its `libraryCount` is `0`, [`stage_count`] **must**  not
///   be `0`
/// - If [`library_info`] is not `NULL` and its `libraryCount` is `0`, [`group_count`] **must**  not
///   be `0`
/// - Any element of the `pDynamicStates` member of [`dynamic_state`] **must**  be
///   `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`PipelineCreationFeedbackCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`PipelineCreateFlagBits`] values
/// - If [`stage_count`] is not `0`, [`stages`] **must**  be a valid pointer to an array of
///   [`stage_count`] valid [`PipelineShaderStageCreateInfo`] structures
/// - If [`group_count`] is not `0`, [`groups`] **must**  be a valid pointer to an array of
///   [`group_count`] valid [`RayTracingShaderGroupCreateInfoKHR`] structures
/// - If [`library_info`] is not `NULL`, [`library_info`] **must**  be a valid pointer to a valid
///   [`PipelineLibraryCreateInfoKHR`] structure
/// - If [`library_interface`] is not `NULL`, [`library_interface`] **must**  be a valid pointer to
///   a valid [`RayTracingPipelineInterfaceCreateInfoKHR`] structure
/// - If [`dynamic_state`] is not `NULL`, [`dynamic_state`] **must**  be a valid pointer to a valid
///   [`PipelineDynamicStateCreateInfo`] structure
/// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
/// - Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored
///   parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`Pipeline`]
/// - [`PipelineCreateFlags`]
/// - [`PipelineDynamicStateCreateInfo`]
/// - [`PipelineLayout`]
/// - [`PipelineLibraryCreateInfoKHR`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`RayTracingPipelineInterfaceCreateInfoKHR`]
/// - [`RayTracingShaderGroupCreateInfoKHR`]
/// - [`StructureType`]
/// - [`create_ray_tracing_pipelines_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying
    ///how the pipeline will be generated.
    pub flags: PipelineCreateFlags,
    ///[`stage_count`] is the number of entries in the [`stages`] array.
    pub stage_count: u32,
    ///[`stages`] is a pointer to an array of [`stage_count`][`PipelineShaderStageCreateInfo`]
    /// structures describing the set of the shader stages to be included in the ray tracing
    /// pipeline.
    pub stages: *const PipelineShaderStageCreateInfo<'lt>,
    ///[`group_count`] is the number of entries in the [`groups`] array.
    pub group_count: u32,
    ///[`groups`] is a pointer to an array of [`group_count`][`RayTracingShaderGroupCreateInfoKHR`]
    /// structures describing the set of the shader stages to be included in each shader group
    /// in the ray tracing pipeline.
    pub groups: *const RayTracingShaderGroupCreateInfoKHR<'lt>,
    ///[`max_pipeline_ray_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
    pub max_pipeline_ray_recursion_depth: u32,
    ///[`library_info`] is a pointer to a
    ///[`PipelineLibraryCreateInfoKHR`] structure defining pipeline
    ///libraries to include.
    pub library_info: *const PipelineLibraryCreateInfoKHR<'lt>,
    ///[`library_interface`] is a pointer to a
    ///[`RayTracingPipelineInterfaceCreateInfoKHR`] structure defining
    ///additional information when using pipeline libraries.
    pub library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR<'lt>,
    ///[`dynamic_state`] is a pointer to a
    ///[`PipelineDynamicStateCreateInfo`] structure, and is used to
    ///indicate which properties of the pipeline state object are dynamic and
    /// **can**  be changed independently of the pipeline state.
    ///This  **can**  be `NULL`, which means no state in the pipeline is considered
    ///dynamic.
    pub dynamic_state: *const PipelineDynamicStateCreateInfo<'lt>,
    ///[`layout`] is the description of binding locations used by both the
    ///pipeline and descriptor sets used with the pipeline.
    pub layout: PipelineLayout,
    ///[`base_pipeline_handle`] is a pipeline to derive from.
    pub base_pipeline_handle: Pipeline,
    ///[`base_pipeline_index`] is an index into the `pCreateInfos`
    ///parameter to use as a pipeline to derive from.
    pub base_pipeline_index: i32,
}
impl<'lt> Default for RayTracingPipelineCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: 0,
            stages: std::ptr::null(),
            group_count: 0,
            groups: std::ptr::null(),
            max_pipeline_ray_recursion_depth: 0,
            library_info: std::ptr::null(),
            library_interface: std::ptr::null(),
            dynamic_state: std::ptr::null(),
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: 0,
        }
    }
}
impl<'lt> RayTracingPipelineCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stages`]
    pub fn stages_raw(&self) -> *const PipelineShaderStageCreateInfo<'lt> {
        self.stages
    }
    ///Gets the raw value of [`Self::groups`]
    pub fn groups_raw(&self) -> *const RayTracingShaderGroupCreateInfoKHR<'lt> {
        self.groups
    }
    ///Gets the raw value of [`Self::library_info`]
    pub fn library_info_raw(&self) -> *const PipelineLibraryCreateInfoKHR<'lt> {
        self.library_info
    }
    ///Gets the raw value of [`Self::library_interface`]
    pub fn library_interface_raw(&self) -> *const RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
        self.library_interface
    }
    ///Gets the raw value of [`Self::dynamic_state`]
    pub fn dynamic_state_raw(&self) -> *const PipelineDynamicStateCreateInfo<'lt> {
        self.dynamic_state
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stages`]
    pub fn set_stages_raw(&mut self, value: *const PipelineShaderStageCreateInfo<'lt>) -> &mut Self {
        self.stages = value;
        self
    }
    ///Sets the raw value of [`Self::groups`]
    pub fn set_groups_raw(&mut self, value: *const RayTracingShaderGroupCreateInfoKHR<'lt>) -> &mut Self {
        self.groups = value;
        self
    }
    ///Sets the raw value of [`Self::library_info`]
    pub fn set_library_info_raw(&mut self, value: *const PipelineLibraryCreateInfoKHR<'lt>) -> &mut Self {
        self.library_info = value;
        self
    }
    ///Sets the raw value of [`Self::library_interface`]
    pub fn set_library_interface_raw(
        &mut self,
        value: *const RayTracingPipelineInterfaceCreateInfoKHR<'lt>,
    ) -> &mut Self {
        self.library_interface = value;
        self
    }
    ///Sets the raw value of [`Self::dynamic_state`]
    pub fn set_dynamic_state_raw(&mut self, value: *const PipelineDynamicStateCreateInfo<'lt>) -> &mut Self {
        self.dynamic_state = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stages`]
    pub fn with_stages_raw(mut self, value: *const PipelineShaderStageCreateInfo<'lt>) -> Self {
        self.stages = value;
        self
    }
    ///Sets the raw value of [`Self::groups`]
    pub fn with_groups_raw(mut self, value: *const RayTracingShaderGroupCreateInfoKHR<'lt>) -> Self {
        self.groups = value;
        self
    }
    ///Sets the raw value of [`Self::library_info`]
    pub fn with_library_info_raw(mut self, value: *const PipelineLibraryCreateInfoKHR<'lt>) -> Self {
        self.library_info = value;
        self
    }
    ///Sets the raw value of [`Self::library_interface`]
    pub fn with_library_interface_raw(mut self, value: *const RayTracingPipelineInterfaceCreateInfoKHR<'lt>) -> Self {
        self.library_interface = value;
        self
    }
    ///Sets the raw value of [`Self::dynamic_state`]
    pub fn with_dynamic_state_raw(mut self, value: *const PipelineDynamicStateCreateInfo<'lt>) -> Self {
        self.dynamic_state = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::stage_count`]
    pub fn stage_count(&self) -> u32 {
        self.stage_count
    }
    ///Gets the value of [`Self::stages`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn stages(&self) -> &[PipelineShaderStageCreateInfo<'lt>] {
        std::slice::from_raw_parts(self.stages, self.stage_count as usize)
    }
    ///Gets the value of [`Self::group_count`]
    pub fn group_count(&self) -> u32 {
        self.group_count
    }
    ///Gets the value of [`Self::groups`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn groups(&self) -> &[RayTracingShaderGroupCreateInfoKHR<'lt>] {
        std::slice::from_raw_parts(self.groups, self.group_count as usize)
    }
    ///Gets the value of [`Self::max_pipeline_ray_recursion_depth`]
    pub fn max_pipeline_ray_recursion_depth(&self) -> u32 {
        self.max_pipeline_ray_recursion_depth
    }
    ///Gets the value of [`Self::library_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn library_info(&self) -> &PipelineLibraryCreateInfoKHR<'lt> {
        &*self.library_info
    }
    ///Gets the value of [`Self::library_interface`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn library_interface(&self) -> &RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
        &*self.library_interface
    }
    ///Gets the value of [`Self::dynamic_state`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dynamic_state(&self) -> &PipelineDynamicStateCreateInfo<'lt> {
        &*self.dynamic_state
    }
    ///Gets the value of [`Self::layout`]
    pub fn layout(&self) -> PipelineLayout {
        self.layout
    }
    ///Gets the value of [`Self::base_pipeline_handle`]
    pub fn base_pipeline_handle(&self) -> Pipeline {
        self.base_pipeline_handle
    }
    ///Gets the value of [`Self::base_pipeline_index`]
    pub fn base_pipeline_index(&self) -> i32 {
        self.base_pipeline_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::stage_count`]
    pub fn stage_count_mut(&mut self) -> &mut u32 {
        &mut self.stage_count
    }
    ///Gets a mutable reference to the value of [`Self::group_count`]
    pub fn group_count_mut(&mut self) -> &mut u32 {
        &mut self.group_count
    }
    ///Gets a mutable reference to the value of [`Self::max_pipeline_ray_recursion_depth`]
    pub fn max_pipeline_ray_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_recursion_depth
    }
    ///Gets a mutable reference to the value of [`Self::layout`]
    pub fn layout_mut(&mut self) -> &mut PipelineLayout {
        &mut self.layout
    }
    ///Gets a mutable reference to the value of [`Self::base_pipeline_handle`]
    pub fn base_pipeline_handle_mut(&mut self) -> &mut Pipeline {
        &mut self.base_pipeline_handle
    }
    ///Gets a mutable reference to the value of [`Self::base_pipeline_index`]
    pub fn base_pipeline_index_mut(&mut self) -> &mut i32 {
        &mut self.base_pipeline_index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_0::PipelineCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::stage_count`]
    pub fn set_stage_count(&mut self, value: u32) -> &mut Self {
        self.stage_count = value;
        self
    }
    ///Sets the value of [`Self::stages`]
    pub fn set_stages(&mut self, value: &'lt [crate::vulkan1_0::PipelineShaderStageCreateInfo<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.stages = value.as_ptr();
        self.stage_count = len_;
        self
    }
    ///Sets the value of [`Self::group_count`]
    pub fn set_group_count(&mut self, value: u32) -> &mut Self {
        self.group_count = value;
        self
    }
    ///Sets the value of [`Self::groups`]
    pub fn set_groups(
        &mut self,
        value: &'lt [crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.groups = value.as_ptr();
        self.group_count = len_;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_recursion_depth`]
    pub fn set_max_pipeline_ray_recursion_depth(&mut self, value: u32) -> &mut Self {
        self.max_pipeline_ray_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::library_info`]
    pub fn set_library_info(
        &mut self,
        value: &'lt crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR<'lt>,
    ) -> &mut Self {
        self.library_info = value as *const _;
        self
    }
    ///Sets the value of [`Self::library_interface`]
    pub fn set_library_interface(
        &mut self,
        value: &'lt crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR<'lt>,
    ) -> &mut Self {
        self.library_interface = value as *const _;
        self
    }
    ///Sets the value of [`Self::dynamic_state`]
    pub fn set_dynamic_state(
        &mut self,
        value: &'lt crate::vulkan1_0::PipelineDynamicStateCreateInfo<'lt>,
    ) -> &mut Self {
        self.dynamic_state = value as *const _;
        self
    }
    ///Sets the value of [`Self::layout`]
    pub fn set_layout(&mut self, value: crate::vulkan1_0::PipelineLayout) -> &mut Self {
        self.layout = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_handle`]
    pub fn set_base_pipeline_handle(&mut self, value: crate::vulkan1_0::Pipeline) -> &mut Self {
        self.base_pipeline_handle = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_index`]
    pub fn set_base_pipeline_index(&mut self, value: i32) -> &mut Self {
        self.base_pipeline_index = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::vulkan1_0::PipelineCreateFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::stage_count`]
    pub fn with_stage_count(mut self, value: u32) -> Self {
        self.stage_count = value;
        self
    }
    ///Sets the value of [`Self::stages`]
    pub fn with_stages(mut self, value: &'lt [crate::vulkan1_0::PipelineShaderStageCreateInfo<'lt>]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.stages = value.as_ptr();
        self.stage_count = len_;
        self
    }
    ///Sets the value of [`Self::group_count`]
    pub fn with_group_count(mut self, value: u32) -> Self {
        self.group_count = value;
        self
    }
    ///Sets the value of [`Self::groups`]
    pub fn with_groups(
        mut self,
        value: &'lt [crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupCreateInfoKHR<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.groups = value.as_ptr();
        self.group_count = len_;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_recursion_depth`]
    pub fn with_max_pipeline_ray_recursion_depth(mut self, value: u32) -> Self {
        self.max_pipeline_ray_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::library_info`]
    pub fn with_library_info(
        mut self,
        value: &'lt crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR<'lt>,
    ) -> Self {
        self.library_info = value as *const _;
        self
    }
    ///Sets the value of [`Self::library_interface`]
    pub fn with_library_interface(
        mut self,
        value: &'lt crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineInterfaceCreateInfoKHR<'lt>,
    ) -> Self {
        self.library_interface = value as *const _;
        self
    }
    ///Sets the value of [`Self::dynamic_state`]
    pub fn with_dynamic_state(mut self, value: &'lt crate::vulkan1_0::PipelineDynamicStateCreateInfo<'lt>) -> Self {
        self.dynamic_state = value as *const _;
        self
    }
    ///Sets the value of [`Self::layout`]
    pub fn with_layout(mut self, value: crate::vulkan1_0::PipelineLayout) -> Self {
        self.layout = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_handle`]
    pub fn with_base_pipeline_handle(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.base_pipeline_handle = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_index`]
    pub fn with_base_pipeline_index(mut self, value: i32) -> Self {
        self.base_pipeline_index = value;
        self
    }
}
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, PipelineCreationFeedbackCreateInfo<'extender>> for RayTracingPipelineCreateInfoKHR<'this>
{
    type Out = RayTracingPipelineCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut PipelineCreationFeedbackCreateInfo<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut PipelineCreationFeedbackCreateInfo<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html) - Structure describing the ray tracing features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rayTracingPipeline;
///    VkBool32           rayTracingPipelineShaderGroupHandleCaptureReplay;
///    VkBool32           rayTracingPipelineShaderGroupHandleCaptureReplayMixed;
///    VkBool32           rayTracingPipelineTraceRaysIndirect;
///    VkBool32           rayTraversalPrimitiveCulling;
///} VkPhysicalDeviceRayTracingPipelineFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ray_tracing_pipeline`] indicates whether the implementation supports the ray tracing pipeline functionality. See [Ray Tracing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing).
/// - [`ray_tracing_pipeline_shader_group_handle_capture_replay`] indicates whether the
///   implementation supports saving and reusing shader group handles, e.g. for trace capture and
///   replay.
/// - [`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] indicates whether the
///   implementation supports reuse of shader group handles being arbitrarily mixed with creation of
///   non-reused shader group handles. If this is [`FALSE`], all reused shader group handles
///   **must**  be specified before any non-reused handles  **may**  be created.
/// - [`ray_tracing_pipeline_trace_rays_indirect`] indicates whether the implementation supports
///   indirect ray tracing commands, e.g. [`cmd_trace_rays_indirect_khr`].
/// - [`ray_traversal_primitive_culling`] indicates whether the implementation supports [primitive culling during ray traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-primitive).
///If the [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayTracingPipelineFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage
/// - If [`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`TRUE`],
///   [`ray_tracing_pipeline_shader_group_handle_capture_replay`] **must**  also be [`TRUE`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`ray_tracing_pipeline`] indicates
    ///whether the implementation supports the ray tracing pipeline
    ///functionality.
    ///See [Ray Tracing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing).
    pub ray_tracing_pipeline: Bool32,
    ///[`ray_tracing_pipeline_shader_group_handle_capture_replay`] indicates whether
    ///the implementation supports saving and reusing shader group handles,
    ///e.g. for trace capture and replay.
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    ///[`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] indicates
    ///whether the implementation supports reuse of shader group handles being
    ///arbitrarily mixed with creation of non-reused shader group handles.
    ///If this is [`FALSE`], all reused shader group handles  **must**  be
    ///specified before any non-reused handles  **may**  be created.
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    ///[`ray_tracing_pipeline_trace_rays_indirect`] indicates whether the
    ///implementation supports indirect ray tracing commands, e.g.
    ///[`cmd_trace_rays_indirect_khr`].
    pub ray_tracing_pipeline_trace_rays_indirect: Bool32,
    ///[`ray_traversal_primitive_culling`] indicates whether the implementation
    ///supports [primitive culling during ray
    ///traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-primitive).
    pub ray_traversal_primitive_culling: Bool32,
}
impl<'lt> Default for PhysicalDeviceRayTracingPipelineFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_tracing_pipeline: 0,
            ray_tracing_pipeline_shader_group_handle_capture_replay: 0,
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: 0,
            ray_tracing_pipeline_trace_rays_indirect: 0,
            ray_traversal_primitive_culling: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRayTracingPipelineFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::ray_tracing_pipeline`]
    pub fn ray_tracing_pipeline_raw(&self) -> Bool32 {
        self.ray_tracing_pipeline
    }
    ///Gets the raw value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_raw(&self) -> Bool32 {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay
    }
    ///Gets the raw value of
    /// [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed_raw(&self) -> Bool32 {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
    }
    ///Gets the raw value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_pipeline_trace_rays_indirect_raw(&self) -> Bool32 {
        self.ray_tracing_pipeline_trace_rays_indirect
    }
    ///Gets the raw value of [`Self::ray_traversal_primitive_culling`]
    pub fn ray_traversal_primitive_culling_raw(&self) -> Bool32 {
        self.ray_traversal_primitive_culling
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline`]
    pub fn set_ray_tracing_pipeline_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_tracing_pipeline = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn set_ray_tracing_pipeline_trace_rays_indirect_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_tracing_pipeline_trace_rays_indirect = value;
        self
    }
    ///Sets the raw value of [`Self::ray_traversal_primitive_culling`]
    pub fn set_ray_traversal_primitive_culling_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_traversal_primitive_culling = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline`]
    pub fn with_ray_tracing_pipeline_raw(mut self, value: Bool32) -> Self {
        self.ray_tracing_pipeline = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay_raw(mut self, value: Bool32) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed_raw(mut self, value: Bool32) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = value;
        self
    }
    ///Sets the raw value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn with_ray_tracing_pipeline_trace_rays_indirect_raw(mut self, value: Bool32) -> Self {
        self.ray_tracing_pipeline_trace_rays_indirect = value;
        self
    }
    ///Sets the raw value of [`Self::ray_traversal_primitive_culling`]
    pub fn with_ray_traversal_primitive_culling_raw(mut self, value: Bool32) -> Self {
        self.ray_traversal_primitive_culling = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::ray_tracing_pipeline`]
    pub fn ray_tracing_pipeline(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_pipeline as u8) }
    }
    ///Gets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_pipeline_shader_group_handle_capture_replay as u8) }
    }
    ///Gets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed as u8) }
    }
    ///Gets the value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_pipeline_trace_rays_indirect(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_tracing_pipeline_trace_rays_indirect as u8) }
    }
    ///Gets the value of [`Self::ray_traversal_primitive_culling`]
    pub fn ray_traversal_primitive_culling(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_traversal_primitive_culling as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::ray_tracing_pipeline`]
    pub fn ray_tracing_pipeline_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_pipeline as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_pipeline as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_pipeline_shader_group_handle_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_pipeline_shader_group_handle_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn ray_tracing_pipeline_trace_rays_indirect_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_tracing_pipeline_trace_rays_indirect as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_tracing_pipeline_trace_rays_indirect as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::ray_traversal_primitive_culling`]
    pub fn ray_traversal_primitive_culling_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_traversal_primitive_culling as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_traversal_primitive_culling as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline`]
    pub fn set_ray_tracing_pipeline(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_pipeline = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn set_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn set_ray_tracing_pipeline_trace_rays_indirect(&mut self, value: bool) -> &mut Self {
        self.ray_tracing_pipeline_trace_rays_indirect = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_traversal_primitive_culling`]
    pub fn set_ray_traversal_primitive_culling(&mut self, value: bool) -> &mut Self {
        self.ray_traversal_primitive_culling = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline`]
    pub fn with_ray_tracing_pipeline(mut self, value: bool) -> Self {
        self.ray_tracing_pipeline = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay(mut self, value: bool) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`]
    pub fn with_ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(mut self, value: bool) -> Self {
        self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_tracing_pipeline_trace_rays_indirect`]
    pub fn with_ray_tracing_pipeline_trace_rays_indirect(mut self, value: bool) -> Self {
        self.ray_tracing_pipeline_trace_rays_indirect = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::ray_traversal_primitive_culling`]
    pub fn with_ray_traversal_primitive_culling(mut self, value: bool) -> Self {
        self.ray_traversal_primitive_culling = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html) - Properties of the physical device for ray tracing
///# C Specifications
///The [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           shaderGroupHandleSize;
///    uint32_t           maxRayRecursionDepth;
///    uint32_t           maxShaderGroupStride;
///    uint32_t           shaderGroupBaseAlignment;
///    uint32_t           shaderGroupHandleCaptureReplaySize;
///    uint32_t           maxRayDispatchInvocationCount;
///    uint32_t           shaderGroupHandleAlignment;
///    uint32_t           maxRayHitAttributeSize;
///} VkPhysicalDeviceRayTracingPipelinePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_group_handle_size`] is the size in bytes of the shader header.
/// - [`max_ray_recursion_depth`] is the maximum number of levels of ray recursion allowed in a
///   trace command.
/// - [`max_shader_group_stride`] is the maximum stride in bytes allowed between shader groups in
///   the shader binding table.
/// - [`shader_group_base_alignment`] is the  **required**  alignment in bytes for the base of the
///   shader binding table.
/// - [`shader_group_handle_capture_replay_size`] is the number of bytes for the information
///   required to do capture and replay for shader group handles.
/// - [`max_ray_dispatch_invocation_count`] is the maximum number of ray generation shader
///   invocations which  **may**  be produced by a single [`cmd_trace_rays_indirect_khr`] or
///   [`cmd_trace_rays_khr`] command.
/// - [`shader_group_handle_alignment`] is the  **required**  alignment in bytes for each shader
///   binding table entry. The value  **must**  be a power of two.
/// - [`max_ray_hit_attribute_size`] is the maximum size in bytes for a ray attribute structure
///# Description
///If the [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure  **must**
/// match those specified with the same
///name in [`PhysicalDeviceRayTracingPropertiesNV`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_group_handle_size`] is the size in bytes of the shader header.
    pub shader_group_handle_size: u32,
    ///[`max_ray_recursion_depth`] is the
    ///maximum number of levels of ray recursion allowed in a trace command.
    pub max_ray_recursion_depth: u32,
    ///[`max_shader_group_stride`] is the maximum stride in bytes allowed
    ///between shader groups in the shader binding table.
    pub max_shader_group_stride: u32,
    ///[`shader_group_base_alignment`] is the  **required**  alignment in bytes for
    ///the base of the shader binding table.
    pub shader_group_base_alignment: u32,
    ///[`shader_group_handle_capture_replay_size`] is the number of bytes for the
    ///information required to do capture and replay for shader group handles.
    pub shader_group_handle_capture_replay_size: u32,
    ///[`max_ray_dispatch_invocation_count`] is the maximum number of ray
    ///generation shader invocations which  **may**  be produced by a single
    ///[`cmd_trace_rays_indirect_khr`] or [`cmd_trace_rays_khr`] command.
    pub max_ray_dispatch_invocation_count: u32,
    ///[`shader_group_handle_alignment`] is the  **required**  alignment in bytes for
    ///each shader binding table entry.
    ///The value  **must**  be a power of two.
    pub shader_group_handle_alignment: u32,
    ///[`max_ray_hit_attribute_size`] is the maximum size in bytes for a ray
    ///attribute structure
    pub max_ray_hit_attribute_size: u32,
}
impl<'lt> Default for PhysicalDeviceRayTracingPipelinePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: 0,
            max_ray_recursion_depth: 0,
            max_shader_group_stride: 0,
            shader_group_base_alignment: 0,
            shader_group_handle_capture_replay_size: 0,
            max_ray_dispatch_invocation_count: 0,
            shader_group_handle_alignment: 0,
            max_ray_hit_attribute_size: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRayTracingPipelinePropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::shader_group_handle_size`]
    pub fn shader_group_handle_size(&self) -> u32 {
        self.shader_group_handle_size
    }
    ///Gets the value of [`Self::max_ray_recursion_depth`]
    pub fn max_ray_recursion_depth(&self) -> u32 {
        self.max_ray_recursion_depth
    }
    ///Gets the value of [`Self::max_shader_group_stride`]
    pub fn max_shader_group_stride(&self) -> u32 {
        self.max_shader_group_stride
    }
    ///Gets the value of [`Self::shader_group_base_alignment`]
    pub fn shader_group_base_alignment(&self) -> u32 {
        self.shader_group_base_alignment
    }
    ///Gets the value of [`Self::shader_group_handle_capture_replay_size`]
    pub fn shader_group_handle_capture_replay_size(&self) -> u32 {
        self.shader_group_handle_capture_replay_size
    }
    ///Gets the value of [`Self::max_ray_dispatch_invocation_count`]
    pub fn max_ray_dispatch_invocation_count(&self) -> u32 {
        self.max_ray_dispatch_invocation_count
    }
    ///Gets the value of [`Self::shader_group_handle_alignment`]
    pub fn shader_group_handle_alignment(&self) -> u32 {
        self.shader_group_handle_alignment
    }
    ///Gets the value of [`Self::max_ray_hit_attribute_size`]
    pub fn max_ray_hit_attribute_size(&self) -> u32 {
        self.max_ray_hit_attribute_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::shader_group_handle_size`]
    pub fn shader_group_handle_size_mut(&mut self) -> &mut u32 {
        &mut self.shader_group_handle_size
    }
    ///Gets a mutable reference to the value of [`Self::max_ray_recursion_depth`]
    pub fn max_ray_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_ray_recursion_depth
    }
    ///Gets a mutable reference to the value of [`Self::max_shader_group_stride`]
    pub fn max_shader_group_stride_mut(&mut self) -> &mut u32 {
        &mut self.max_shader_group_stride
    }
    ///Gets a mutable reference to the value of [`Self::shader_group_base_alignment`]
    pub fn shader_group_base_alignment_mut(&mut self) -> &mut u32 {
        &mut self.shader_group_base_alignment
    }
    ///Gets a mutable reference to the value of [`Self::shader_group_handle_capture_replay_size`]
    pub fn shader_group_handle_capture_replay_size_mut(&mut self) -> &mut u32 {
        &mut self.shader_group_handle_capture_replay_size
    }
    ///Gets a mutable reference to the value of [`Self::max_ray_dispatch_invocation_count`]
    pub fn max_ray_dispatch_invocation_count_mut(&mut self) -> &mut u32 {
        &mut self.max_ray_dispatch_invocation_count
    }
    ///Gets a mutable reference to the value of [`Self::shader_group_handle_alignment`]
    pub fn shader_group_handle_alignment_mut(&mut self) -> &mut u32 {
        &mut self.shader_group_handle_alignment
    }
    ///Gets a mutable reference to the value of [`Self::max_ray_hit_attribute_size`]
    pub fn max_ray_hit_attribute_size_mut(&mut self) -> &mut u32 {
        &mut self.max_ray_hit_attribute_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_size`]
    pub fn set_shader_group_handle_size(&mut self, value: u32) -> &mut Self {
        self.shader_group_handle_size = value;
        self
    }
    ///Sets the value of [`Self::max_ray_recursion_depth`]
    pub fn set_max_ray_recursion_depth(&mut self, value: u32) -> &mut Self {
        self.max_ray_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::max_shader_group_stride`]
    pub fn set_max_shader_group_stride(&mut self, value: u32) -> &mut Self {
        self.max_shader_group_stride = value;
        self
    }
    ///Sets the value of [`Self::shader_group_base_alignment`]
    pub fn set_shader_group_base_alignment(&mut self, value: u32) -> &mut Self {
        self.shader_group_base_alignment = value;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_capture_replay_size`]
    pub fn set_shader_group_handle_capture_replay_size(&mut self, value: u32) -> &mut Self {
        self.shader_group_handle_capture_replay_size = value;
        self
    }
    ///Sets the value of [`Self::max_ray_dispatch_invocation_count`]
    pub fn set_max_ray_dispatch_invocation_count(&mut self, value: u32) -> &mut Self {
        self.max_ray_dispatch_invocation_count = value;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_alignment`]
    pub fn set_shader_group_handle_alignment(&mut self, value: u32) -> &mut Self {
        self.shader_group_handle_alignment = value;
        self
    }
    ///Sets the value of [`Self::max_ray_hit_attribute_size`]
    pub fn set_max_ray_hit_attribute_size(&mut self, value: u32) -> &mut Self {
        self.max_ray_hit_attribute_size = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_size`]
    pub fn with_shader_group_handle_size(mut self, value: u32) -> Self {
        self.shader_group_handle_size = value;
        self
    }
    ///Sets the value of [`Self::max_ray_recursion_depth`]
    pub fn with_max_ray_recursion_depth(mut self, value: u32) -> Self {
        self.max_ray_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::max_shader_group_stride`]
    pub fn with_max_shader_group_stride(mut self, value: u32) -> Self {
        self.max_shader_group_stride = value;
        self
    }
    ///Sets the value of [`Self::shader_group_base_alignment`]
    pub fn with_shader_group_base_alignment(mut self, value: u32) -> Self {
        self.shader_group_base_alignment = value;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_capture_replay_size`]
    pub fn with_shader_group_handle_capture_replay_size(mut self, value: u32) -> Self {
        self.shader_group_handle_capture_replay_size = value;
        self
    }
    ///Sets the value of [`Self::max_ray_dispatch_invocation_count`]
    pub fn with_max_ray_dispatch_invocation_count(mut self, value: u32) -> Self {
        self.max_ray_dispatch_invocation_count = value;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_alignment`]
    pub fn with_shader_group_handle_alignment(mut self, value: u32) -> Self {
        self.shader_group_handle_alignment = value;
        self
    }
    ///Sets the value of [`Self::max_ray_hit_attribute_size`]
    pub fn with_max_ray_hit_attribute_size(mut self, value: u32) -> Self {
        self.max_ray_hit_attribute_size = value;
        self
    }
}
///[VkStridedDeviceAddressRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html) - Structure specifying a region of device addresses with a stride
///# C Specifications
///The [`StridedDeviceAddressRegionKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkStridedDeviceAddressRegionKHR {
///    VkDeviceAddress    deviceAddress;
///    VkDeviceSize       stride;
///    VkDeviceSize       size;
///} VkStridedDeviceAddressRegionKHR;
///```
///# Members
/// - [`device_address`] is the device address (as returned by the [`get_buffer_device_address`]
///   command) at which the region starts, or zero if the region is unused.
/// - [`stride`] is the byte stride between consecutive elements.
/// - [`size`] is the size in bytes of the region starting at [`device_address`].
///# Description
///## Valid Usage
/// - If [`size`] is not zero, all addresses between [`device_address`] and [`device_address`] +
///   [`size`] - 1 **must**  be in the buffer device address range of the same buffer
/// - If [`size`] is not zero, [`stride`] **must**  be less than or equal to the size of the buffer
///   from which [`device_address`] was queried
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`cmd_trace_rays_indirect_khr`]
/// - [`cmd_trace_rays_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    ///[`device_address`] is the device address (as returned by the
    ///[`get_buffer_device_address`] command) at which the region starts, or
    ///zero if the region is unused.
    pub device_address: DeviceAddress,
    ///[`stride`] is the byte stride between consecutive elements.
    pub stride: DeviceSize,
    ///[`size`] is the size in bytes of the region starting at
    ///[`device_address`].
    pub size: DeviceSize,
}
impl Default for StridedDeviceAddressRegionKHR {
    fn default() -> Self {
        Self {
            device_address: Default::default(),
            stride: Default::default(),
            size: Default::default(),
        }
    }
}
impl StridedDeviceAddressRegionKHR {
    ///Gets the value of [`Self::device_address`]
    pub fn device_address(&self) -> DeviceAddress {
        self.device_address
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> DeviceSize {
        self.stride
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
    }
    ///Gets a mutable reference to the value of [`Self::device_address`]
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.stride
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Sets the value of [`Self::device_address`]
    pub fn set_device_address(&mut self, value: crate::vulkan1_0::DeviceAddress) -> &mut Self {
        self.device_address = value;
        self
    }
    ///Sets the value of [`Self::stride`]
    pub fn set_stride(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.stride = value;
        self
    }
    ///Sets the value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
    ///Sets the value of [`Self::device_address`]
    pub fn with_device_address(mut self, value: crate::vulkan1_0::DeviceAddress) -> Self {
        self.device_address = value;
        self
    }
    ///Sets the value of [`Self::stride`]
    pub fn with_stride(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.stride = value;
        self
    }
    ///Sets the value of [`Self::size`]
    pub fn with_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.size = value;
        self
    }
}
///[VkTraceRaysIndirectCommandKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) - Structure specifying the parameters of an indirect ray tracing command
///# C Specifications
///The [`TraceRaysIndirectCommandKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkTraceRaysIndirectCommandKHR {
///    uint32_t    width;
///    uint32_t    height;
///    uint32_t    depth;
///} VkTraceRaysIndirectCommandKHR;
///```
///# Members
/// - [`width`] is the width of the ray trace query dimensions.
/// - [`height`] is height of the ray trace query dimensions.
/// - [`depth`] is depth of the ray trace query dimensions.
///# Description
///The members of [`TraceRaysIndirectCommandKHR`] have the same meaning as
///the similarly named parameters of [`cmd_trace_rays_khr`].
///## Valid Usage
/// - [`width`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][0]
/// - [`height`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][1]
/// - [`depth`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][2]
/// - [`width`] × [`height`] × [`depth`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_dispatch_invocation_count`]
///# Related
/// - [`khr_ray_tracing_pipeline`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    ///[`width`] is the width of the ray trace query dimensions.
    pub width: u32,
    ///[`height`] is height of the ray trace query dimensions.
    pub height: u32,
    ///[`depth`] is depth of the ray trace query dimensions.
    pub depth: u32,
}
impl Default for TraceRaysIndirectCommandKHR {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
        }
    }
}
impl TraceRaysIndirectCommandKHR {
    ///Gets the value of [`Self::width`]
    pub fn width(&self) -> u32 {
        self.width
    }
    ///Gets the value of [`Self::height`]
    pub fn height(&self) -> u32 {
        self.height
    }
    ///Gets the value of [`Self::depth`]
    pub fn depth(&self) -> u32 {
        self.depth
    }
    ///Gets a mutable reference to the value of [`Self::width`]
    pub fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    ///Gets a mutable reference to the value of [`Self::height`]
    pub fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }
    ///Gets a mutable reference to the value of [`Self::depth`]
    pub fn depth_mut(&mut self) -> &mut u32 {
        &mut self.depth
    }
    ///Sets the value of [`Self::width`]
    pub fn set_width(&mut self, value: u32) -> &mut Self {
        self.width = value;
        self
    }
    ///Sets the value of [`Self::height`]
    pub fn set_height(&mut self, value: u32) -> &mut Self {
        self.height = value;
        self
    }
    ///Sets the value of [`Self::depth`]
    pub fn set_depth(&mut self, value: u32) -> &mut Self {
        self.depth = value;
        self
    }
    ///Sets the value of [`Self::width`]
    pub fn with_width(mut self, value: u32) -> Self {
        self.width = value;
        self
    }
    ///Sets the value of [`Self::height`]
    pub fn with_height(mut self, value: u32) -> Self {
        self.height = value;
        self
    }
    ///Sets the value of [`Self::depth`]
    pub fn with_depth(mut self, value: u32) -> Self {
        self.depth = value;
        self
    }
}
///[VkRayTracingPipelineInterfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) - Structure specifying additional interface information when using libraries
///# C Specifications
///The [`RayTracingPipelineInterfaceCreateInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_ray_tracing_pipeline
///typedef struct VkRayTracingPipelineInterfaceCreateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           maxPipelineRayPayloadSize;
///    uint32_t           maxPipelineRayHitAttributeSize;
///} VkRayTracingPipelineInterfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_pipeline_ray_payload_size`] is the maximum payload size in bytes used by any shader in
///   the pipeline.
/// - [`max_pipeline_ray_hit_attribute_size`] is the maximum attribute structure size in bytes used
///   by any shader in the pipeline.
///# Description
///[`max_pipeline_ray_payload_size`] is calculated as the maximum number of bytes
///used by any block declared in the `RayPayloadKHR` or
///`IncomingRayPayloadKHR` storage classes.
///[`max_pipeline_ray_hit_attribute_size`] is calculated as the maximum number of
///bytes used by any block declared in the `HitAttributeKHR` storage class.
///As variables in these storage classes do not have explicit offsets, the size
///should be calculated as if each variable has a
///[scalar alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-alignment-requirements) equal to the largest
///scalar alignment of any of the block’s members.
///## Valid Usage
/// - [`max_pipeline_ray_hit_attribute_size`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_hit_attribute_size`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`khr_ray_tracing_pipeline`]
/// - [`RayTracingPipelineCreateInfoKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`max_pipeline_ray_payload_size`] is the maximum payload size in bytes
    ///used by any shader in the pipeline.
    pub max_pipeline_ray_payload_size: u32,
    ///[`max_pipeline_ray_hit_attribute_size`] is the maximum attribute structure
    ///size in bytes used by any shader in the pipeline.
    pub max_pipeline_ray_hit_attribute_size: u32,
}
impl<'lt> Default for RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            max_pipeline_ray_payload_size: 0,
            max_pipeline_ray_hit_attribute_size: 0,
        }
    }
}
impl<'lt> RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::max_pipeline_ray_payload_size`]
    pub fn max_pipeline_ray_payload_size(&self) -> u32 {
        self.max_pipeline_ray_payload_size
    }
    ///Gets the value of [`Self::max_pipeline_ray_hit_attribute_size`]
    pub fn max_pipeline_ray_hit_attribute_size(&self) -> u32 {
        self.max_pipeline_ray_hit_attribute_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_pipeline_ray_payload_size`]
    pub fn max_pipeline_ray_payload_size_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_payload_size
    }
    ///Gets a mutable reference to the value of [`Self::max_pipeline_ray_hit_attribute_size`]
    pub fn max_pipeline_ray_hit_attribute_size_mut(&mut self) -> &mut u32 {
        &mut self.max_pipeline_ray_hit_attribute_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_payload_size`]
    pub fn set_max_pipeline_ray_payload_size(&mut self, value: u32) -> &mut Self {
        self.max_pipeline_ray_payload_size = value;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_hit_attribute_size`]
    pub fn set_max_pipeline_ray_hit_attribute_size(&mut self, value: u32) -> &mut Self {
        self.max_pipeline_ray_hit_attribute_size = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_payload_size`]
    pub fn with_max_pipeline_ray_payload_size(mut self, value: u32) -> Self {
        self.max_pipeline_ray_payload_size = value;
        self
    }
    ///Sets the value of [`Self::max_pipeline_ray_hit_attribute_size`]
    pub fn with_max_pipeline_ray_hit_attribute_size(mut self, value: u32) -> Self {
        self.max_pipeline_ray_hit_attribute_size = value;
        self
    }
}
impl Device {
    ///[vkGetRayTracingShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html) - Query ray tracing pipeline shader group handles
    ///# C Specifications
    ///To query the opaque handles of shaders in the ray tracing pipeline, call:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///VkResult vkGetRayTracingShaderGroupHandlesKHR(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    firstGroup,
    ///    uint32_t                                    groupCount,
    ///    size_t                                      dataSize,
    ///    void*                                       pData);
    ///```
    ///or the equivalent command
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkGetRayTracingShaderGroupHandlesNV(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    firstGroup,
    ///    uint32_t                                    groupCount,
    ///    size_t                                      dataSize,
    ///    void*                                       pData);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device containing the ray tracing pipeline.
    /// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
    /// - [`first_group`] is the index of the first group to retrieve a handle     for from the
    ///   [`RayTracingPipelineCreateInfoKHR::groups`] or [`RayTracingPipelineCreateInfoNV::groups`]
    ///   array.
    /// - [`group_count`] is the number of shader handles to retrieve.
    /// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
    /// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
    ///# Description
    ///## Valid Usage
    /// - [`pipeline`] **must**  be a ray tracing pipeline
    /// - [`first_group`] **must**  be less than the number of shader groups in [`pipeline`]
    /// - The sum of [`first_group`] and [`group_count`] **must**  be less than or equal to the
    ///   number of shader groups in [`pipeline`]
    /// - [`data_size`] **must**  be at least
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_size`] ×
    ///   [`group_count`]
    /// - [`pipeline`] **must**  have not been created with `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
    /// - [`data_size`] **must**  be greater than `0`
    /// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`nv_ray_tracing`]
    /// - [`Device`]
    /// - [`Pipeline`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
    #[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        self: &Unique<Device>,
        pipeline: Pipeline,
        first_group: Option<u32>,
        group_count: Option<u32>,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_shader_group_handles_khr())
            .or_else(|| {
                #[cfg(feature = "VK_NV_ray_tracing")]
                return self
                    .vtable()
                    .nv_ray_tracing()
                    .and_then(|vtable| vtable.get_ray_tracing_shader_group_handles_nv());
                #[cfg(not(feature = "VK_NV_ray_tracing"))]
                return None;
            })
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_shader_group_handles_khr())
            .or_else(|| {
                #[cfg(feature = "VK_NV_ray_tracing")]
                return self
                    .vtable()
                    .nv_ray_tracing()
                    .and_then(|vtable| vtable.get_ray_tracing_shader_group_handles_nv());
                #[cfg(not(feature = "VK_NV_ray_tracing"))]
                return None;
            })
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            pipeline,
            first_group.unwrap_or_default() as _,
            group_count.unwrap_or_default() as _,
            data_size,
            p_data,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) - Query ray tracing capture replay pipeline shader group handles
    ///# C Specifications
    ///To query the optional capture handle information of shaders in the ray
    ///tracing pipeline, call:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///VkResult vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    firstGroup,
    ///    uint32_t                                    groupCount,
    ///    size_t                                      dataSize,
    ///    void*                                       pData);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device containing the ray tracing pipeline.
    /// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
    /// - [`first_group`] is the index of the first group to retrieve a handle for from the
    ///   [`RayTracingPipelineCreateInfoKHR::groups`] array.
    /// - [`group_count`] is the number of shader handles to retrieve.
    /// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
    /// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
    ///# Description
    ///## Valid Usage
    /// - [`pipeline`] **must**  be a ray tracing pipeline
    /// - [`first_group`] **must**  be less than the number of shader groups in [`pipeline`]
    /// - The sum of [`first_group`] and [`group_count`] **must**  be less than or equal to the
    ///   number of shader groups in [`pipeline`]
    /// - [`data_size`] **must**  be at least
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_capture_replay_size`]
    ///   × [`group_count`]
    /// -  [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] **must**  be enabled to call this function
    /// - [`pipeline`] **must**  have been created with a `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
    /// - [`data_size`] **must**  be greater than `0`
    /// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`Device`]
    /// - [`Pipeline`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        self: &Unique<Device>,
        pipeline: Pipeline,
        first_group: Option<u32>,
        group_count: Option<u32>,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_capture_replay_shader_group_handles_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_capture_replay_shader_group_handles_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            pipeline,
            first_group.unwrap_or_default() as _,
            group_count.unwrap_or_default() as _,
            data_size,
            p_data,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkCreateRayTracingPipelinesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html) - Creates a new ray tracing pipeline object
    ///# C Specifications
    ///To create ray tracing pipelines, call:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///VkResult vkCreateRayTracingPipelinesKHR(
    ///    VkDevice                                    device,
    ///    VkDeferredOperationKHR                      deferredOperation,
    ///    VkPipelineCache                             pipelineCache,
    ///    uint32_t                                    createInfoCount,
    ///    const VkRayTracingPipelineCreateInfoKHR*    pCreateInfos,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkPipeline*                                 pPipelines);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the ray tracing pipelines.
    /// - [`deferred_operation`] is [`crate::Handle::null`] or the handle of a valid [`DeferredOperationKHR`][request deferral](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations-requesting) object for this command.
    /// - [`pipeline_cache`] is either [`crate::Handle::null`], indicating that pipeline caching is disabled, or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache) object, in which case use of that cache is enabled for the duration of the command.
    /// - [`create_info_count`] is the length of the [`p_create_infos`] and [`p_pipelines`] arrays.
    /// - [`p_create_infos`] is a pointer to an array of [`RayTracingPipelineCreateInfoKHR`]
    ///   structures.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_pipelines`] is a pointer to an array in which the resulting ray tracing pipeline
    ///   objects are returned.
    ///# Description
    ///The `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` error is returned if the
    ///implementation is unable to re-use the shader group handles provided in
    ///[`RayTracingShaderGroupCreateInfoKHR::shader_group_capture_replay_handle`]
    ///when
    ///[`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`]
    ///is enabled.
    ///## Valid Usage
    /// - If the `flags` member of any element of [`p_create_infos`] contains the
    ///   `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and the `basePipelineIndex` member of that same
    ///   element is not `-1`, `basePipelineIndex` **must**  be less than the index into
    ///   [`p_create_infos`] that corresponds to that element
    /// - If the `flags` member of any element of [`p_create_infos`] contains the
    ///   `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, the base pipeline  **must**  have been created
    ///   with the `VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT` flag set
    /// - `flags` **must**  not contain the `VK_PIPELINE_CREATE_DISPATCH_BASE` flag
    /// -    If [`pipeline_cache`] was created with `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`, host access to [`pipeline_cache`] **must**  be [externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#fundamentals-threadingbehavior)
    ///
    /// - If [`deferred_operation`] is not [`crate::Handle::null`], it  **must**  be a valid
    ///   [`DeferredOperationKHR`] object
    /// - Any previous deferred operation that was associated with [`deferred_operation`] **must**
    ///   be complete
    /// - The [`rayTracingPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipeline)
    ///   feature  **must**  be enabled
    /// - If [`deferred_operation`] is not [`crate::Handle::null`], the `flags` member of elements
    ///   of [`p_create_infos`] **must**  not include
    ///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`deferred_operation`] is not [`crate::Handle::null`], [`deferred_operation`] **must**
    ///   be a valid [`DeferredOperationKHR`] handle
    /// - If [`pipeline_cache`] is not [`crate::Handle::null`], [`pipeline_cache`] **must**  be a
    ///   valid [`PipelineCache`] handle
    /// - [`p_create_infos`] **must**  be a valid pointer to an array of [`create_info_count`] valid
    ///   [`RayTracingPipelineCreateInfoKHR`] structures
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_pipelines`] **must**  be a valid pointer to an array of
    ///   [`create_info_count`][`Pipeline`] handles
    /// - [`create_info_count`] **must**  be greater than `0`
    /// - If [`deferred_operation`] is a valid handle, it  **must**  have been created, allocated,
    ///   or retrieved from [`device`]
    /// - If [`pipeline_cache`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_OPERATION_DEFERRED_KHR`  - `VK_OPERATION_NOT_DEFERRED_KHR`  -
    ///   `VK_PIPELINE_COMPILE_REQUIRED_EXT`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`AllocationCallbacks`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    /// - [`Pipeline`]
    /// - [`PipelineCache`]
    /// - [`RayTracingPipelineCreateInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateRayTracingPipelinesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_ray_tracing_pipelines_khr<'lt>(
        self: &Unique<Device>,
        deferred_operation: Option<DeferredOperationKHR>,
        pipeline_cache: Option<PipelineCache>,
        p_create_infos: &[crate::extensions::khr_ray_tracing_pipeline::RayTracingPipelineCreateInfoKHR<'lt>],
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<SmallVec<Unique<Pipeline>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.create_ray_tracing_pipelines_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.create_ray_tracing_pipelines_khr())
            .unwrap_unchecked();
        let create_info_count = (|len: usize| len)(p_create_infos.len()) as _;
        let mut p_pipelines = SmallVec::<Pipeline>::from_elem(Default::default(), create_info_count as usize);
        let _return = _function(
            self.as_raw(),
            deferred_operation.unwrap_or_default(),
            pipeline_cache.unwrap_or_default(),
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_pipelines.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS
            | VulkanResultCodes::OPERATION_DEFERRED_KHR
            | VulkanResultCodes::OPERATION_NOT_DEFERRED_KHR
            | VulkanResultCodes::PIPELINE_COMPILE_REQUIRED => VulkanResult::Success(
                _return,
                p_pipelines
                    .into_iter()
                    .map(|i| Unique::new(self, i, AtomicBool::default()))
                    .collect(),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetRayTracingShaderGroupStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) - Query ray tracing pipeline shader group shader stack size
    ///# C Specifications
    ///To query the pipeline stack size of shaders in a shader group in the ray
    ///tracing pipeline, call:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///VkDeviceSize vkGetRayTracingShaderGroupStackSizeKHR(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    group,
    ///    VkShaderGroupShaderKHR                      groupShader);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device containing the ray tracing pipeline.
    /// - [`pipeline`] is the ray tracing pipeline object containing the shaders groups.
    /// - [`group`] is the index of the shader group to query.
    /// - [`group_shader`] is the type of shader from the group to query.
    ///# Description
    ///The return value is the ray tracing pipeline stack size in bytes for the
    ///specified shader as called from the specified shader group.
    ///## Valid Usage
    /// - [`pipeline`] **must**  be a ray tracing pipeline
    /// - The value of [`group`] must be less than the number of shader groups in [`pipeline`]
    /// - The shader identified by [`group_shader`] in [`group`] **must**  not be
    ///   [`SHADER_UNUSED_KHR`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`group_shader`] **must**  be a valid [`ShaderGroupShaderKHR`] value
    /// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`Device`]
    /// - [`Pipeline`]
    /// - [`ShaderGroupShaderKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        self: &Unique<Device>,
        pipeline: Pipeline,
        group: Option<u32>,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_shader_group_stack_size_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.get_ray_tracing_shader_group_stack_size_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), pipeline, group.unwrap_or_default() as _, group_shader);
        _return
    }
}
impl CommandBuffer {
    ///[vkCmdTraceRaysKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html) - Initialize a ray tracing dispatch
    ///# C Specifications
    ///To dispatch ray tracing use:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///void vkCmdTraceRaysKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkStridedDeviceAddressRegionKHR*      pRaygenShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pMissShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pHitShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pCallableShaderBindingTable,
    ///    uint32_t                                    width,
    ///    uint32_t                                    height,
    ///    uint32_t                                    depth);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`p_raygen_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the ray generation shader stage.
    /// - [`p_miss_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the miss shader stage.
    /// - [`p_hit_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the hit shader stage.
    /// - [`p_callable_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the callable shader stage.
    /// - [`width`] is the width of the ray trace query dimensions.
    /// - [`height`] is height of the ray trace query dimensions.
    /// - [`depth`] is depth of the ray trace query dimensions.
    ///# Description
    ///When the command is executed, a ray generation group of [`width`]
    ///× [`height`] × [`depth`] rays is assembled.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
    ///   so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - Any shader group handle referenced by this call  **must**  have been queried from the
    ///   currently bound ray tracing pipeline
    ///
    /// - This command  **must**  not cause a shader call instruction to be executed from a shader
    ///   invocation with a [recursion depth]() greater than the value of
    ///   `maxPipelineRayRecursionDepth` used to create the bound ray tracing pipeline
    /// - If the buffer from which `pRayGenShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pRayGenShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pRayGenShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `size` member of `pRayGenShaderBindingTable` **must**  be equal to its `stride` member
    /// - If the buffer from which `pMissShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pMissShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pMissShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the buffer from which `pHitShaderBindingTable->deviceAddress` was queried is non-sparse
    ///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`]
    ///   object
    /// - The buffer from which the `pHitShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pHitShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the buffer from which `pCallableShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pCallableShaderBindingTable->deviceAddress` is queried
    ///   **must**  have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage
    ///   flag
    /// - `pCallableShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be less than or equal
    ///   to [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, the `deviceAddress`
    ///   member of [`p_hit_shader_binding_table`] **must**  not be zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, the `deviceAddress`
    ///   member of [`p_hit_shader_binding_table`] **must**  not be zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, the shader group handle
    ///   identified by [`p_miss_shader_binding_table`] **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
    ///   any-hit shader  **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute a
    ///   closest hit shader  **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
    ///   intersection shader  **must**  not be set to zero
    /// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this
    ///   call from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR` **must**
    ///   have been created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR`
    /// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this
    ///   call from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR` **must**  have
    ///   been created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`
    ///
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - [`width`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0] ×
    ///   [`PhysicalDeviceLimits::max_compute_work_group_size`][0]
    /// - [`height`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1] ×
    ///   [`PhysicalDeviceLimits::max_compute_work_group_size`][1]
    /// - [`depth`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2] ×
    ///   [`PhysicalDeviceLimits::max_compute_work_group_size`][2]
    /// - [`width`] × [`height`] × [`depth`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_dispatch_invocation_count`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_raygen_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_miss_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_hit_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_callable_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`CommandBuffer`]
    /// - [`StridedDeviceAddressRegionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdTraceRaysKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_trace_rays_khr(
        self: &Unique<CommandBuffer>,
        p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: Option<u32>,
        height: Option<u32>,
        depth: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_trace_rays_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_trace_rays_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_raygen_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_miss_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_hit_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_callable_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            width.unwrap_or_default() as _,
            height.unwrap_or_default() as _,
            depth.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdTraceRaysIndirectKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html) - Initialize an indirect ray tracing dispatch
    ///# C Specifications
    ///To dispatch ray tracing, with some parameters sourced on the device, use:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///void vkCmdTraceRaysIndirectKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkStridedDeviceAddressRegionKHR*      pRaygenShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pMissShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pHitShaderBindingTable,
    ///    const VkStridedDeviceAddressRegionKHR*      pCallableShaderBindingTable,
    ///    VkDeviceAddress                             indirectDeviceAddress);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`p_raygen_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the ray generation shader stage.
    /// - [`p_miss_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the miss shader stage.
    /// - [`p_hit_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the hit shader stage.
    /// - [`p_callable_shader_binding_table`] is a [`StridedDeviceAddressRegionKHR`] that holds the
    ///   shader binding table data for the callable shader stage.
    /// - [`indirect_device_address`] is a buffer device address which is a pointer to a
    ///   [`TraceRaysIndirectCommandKHR`] structure containing the trace ray parameters.
    ///# Description
    ///[`cmd_trace_rays_indirect_khr`] behaves similarly to [`cmd_trace_rays_khr`]
    ///except that the ray trace query dimensions are read by the device from
    ///[`indirect_device_address`] during execution.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
    ///   so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - Any shader group handle referenced by this call  **must**  have been queried from the
    ///   currently bound ray tracing pipeline
    ///
    /// - This command  **must**  not cause a shader call instruction to be executed from a shader
    ///   invocation with a [recursion depth]() greater than the value of
    ///   `maxPipelineRayRecursionDepth` used to create the bound ray tracing pipeline
    /// - If the buffer from which `pRayGenShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pRayGenShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pRayGenShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `size` member of `pRayGenShaderBindingTable` **must**  be equal to its `stride` member
    /// - If the buffer from which `pMissShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pMissShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pMissShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_miss_shader_binding_table`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the buffer from which `pHitShaderBindingTable->deviceAddress` was queried is non-sparse
    ///   then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`]
    ///   object
    /// - The buffer from which the `pHitShaderBindingTable->deviceAddress` is queried  **must**
    ///   have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage flag
    /// - `pHitShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_hit_shader_binding_table`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the buffer from which `pCallableShaderBindingTable->deviceAddress` was queried is
    ///   non-sparse then it  **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object
    /// - The buffer from which the `pCallableShaderBindingTable->deviceAddress` is queried
    ///   **must**  have been created with the `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` usage
    ///   flag
    /// - `pCallableShaderBindingTable->deviceAddress` **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_base_alignment`]
    /// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_alignment`]
    /// - The `stride` member of [`p_callable_shader_binding_table`] **must**  be less than or equal
    ///   to [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_shader_group_stride`]
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, the `deviceAddress`
    ///   member of [`p_hit_shader_binding_table`] **must**  not be zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, the `deviceAddress`
    ///   member of [`p_hit_shader_binding_table`] **must**  not be zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, the shader group handle
    ///   identified by [`p_miss_shader_binding_table`] **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
    ///   any-hit shader  **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute a
    ///   closest hit shader  **must**  not be set to zero
    /// - If the currently bound ray tracing pipeline was created with `flags` that included
    ///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`, entries in
    ///   [`p_hit_shader_binding_table`] accessed as a result of this command in order to execute an
    ///   intersection shader  **must**  not be set to zero
    /// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this
    ///   call from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR` **must**
    ///   have been created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR`
    /// - Any non-zero hit shader group entries in [`p_hit_shader_binding_table`] accessed by this
    ///   call from a geometry with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR` **must**  have
    ///   been created with `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`
    ///
    /// - If the buffer from which [`indirect_device_address`] was queried is non-sparse then it
    ///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
    /// - The buffer from which [`indirect_device_address`] was queried  **must**  have been created
    ///   with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
    /// - [`indirect_device_address`] **must**  be a multiple of `4`
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - All device addresses between [`indirect_device_address`] and [`indirect_device_address`] +
    ///   `sizeof`([`TraceRaysIndirectCommandKHR`]) - 1 **must**  be in the buffer device address
    ///   range of the same buffer
    /// - The [[`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_trace_rays_indirect`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipelineTraceRaysIndirect)
    ///   feature  **must**  be enabled
    /// -    If the bound ray tracing pipeline was created with `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`[`PhysicalDeviceRayTracingMotionBlurFeaturesNV::ray_tracing_motion_blur_pipeline_trace_rays_indirect`] feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_raygen_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_miss_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_hit_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`p_callable_shader_binding_table`] **must**  be a valid pointer to a valid
    ///   [`StridedDeviceAddressRegionKHR`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`CommandBuffer`]
    /// - [`DeviceAddress`]
    /// - [`StridedDeviceAddressRegionKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdTraceRaysIndirectKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_trace_rays_indirect_khr(
        self: &Unique<CommandBuffer>,
        p_raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_trace_rays_indirect_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_trace_rays_indirect_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_raygen_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_miss_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_hit_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            p_callable_shader_binding_table as *const StridedDeviceAddressRegionKHR,
            indirect_device_address,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdSetRayTracingPipelineStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html) - Set the stack size dynamically for a ray tracing pipeline
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the stack size for a ray
    ///tracing pipeline, call:
    ///```c
    ///// Provided by VK_KHR_ray_tracing_pipeline
    ///void vkCmdSetRayTracingPipelineStackSizeKHR(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    pipelineStackSize);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`pipeline_stack_size`] is the stack size to use for subsequent ray tracing trace
    ///   commands.
    ///# Description
    ///This command sets the stack size for subsequent ray tracing commands when
    ///the ray tracing pipeline is created with
    ///`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, the stack size is computed as described in
    ///[Ray Tracing Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).
    ///## Valid Usage
    /// - [`pipeline_stack_size`] **must**  be large enough for any dynamic execution through the
    ///   shaders in the ray tracing pipeline used by a subsequent trace call
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`khr_ray_tracing_pipeline`]
    /// - [`CommandBuffer`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        self: &Unique<CommandBuffer>,
        pipeline_stack_size: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_set_ray_tracing_pipeline_stack_size_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_ray_tracing_pipeline()
            .and_then(|vtable| vtable.cmd_set_ray_tracing_pipeline_stack_size_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), pipeline_stack_size.unwrap_or_default() as _);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_ray_tracing_pipeline`
pub struct DeviceKhrRayTracingPipelineVTable {
    ///See [`FNGetRayTracingShaderGroupHandlesKhr`] for more information.
    pub get_ray_tracing_shader_group_handles_khr: FNGetRayTracingShaderGroupHandlesKhr,
    ///See [`FNGetRayTracingCaptureReplayShaderGroupHandlesKhr`] for more information.
    pub get_ray_tracing_capture_replay_shader_group_handles_khr: FNGetRayTracingCaptureReplayShaderGroupHandlesKhr,
    ///See [`FNCreateRayTracingPipelinesKhr`] for more information.
    pub create_ray_tracing_pipelines_khr: FNCreateRayTracingPipelinesKhr,
    ///See [`FNGetRayTracingShaderGroupStackSizeKhr`] for more information.
    pub get_ray_tracing_shader_group_stack_size_khr: FNGetRayTracingShaderGroupStackSizeKhr,
    ///See [`FNCmdTraceRaysKhr`] for more information.
    pub cmd_trace_rays_khr: FNCmdTraceRaysKhr,
    ///See [`FNCmdTraceRaysIndirectKhr`] for more information.
    pub cmd_trace_rays_indirect_khr: FNCmdTraceRaysIndirectKhr,
    ///See [`FNCmdSetRayTracingPipelineStackSizeKhr`] for more information.
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: FNCmdSetRayTracingPipelineStackSizeKhr,
}
impl DeviceKhrRayTracingPipelineVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_ray_tracing_shader_group_handles_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetRayTracingShaderGroupHandlesKHR").as_ptr(),
                ))
            },
            get_ray_tracing_capture_replay_shader_group_handles_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR").as_ptr(),
                ))
            },
            create_ray_tracing_pipelines_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateRayTracingPipelinesKHR").as_ptr(),
                ))
            },
            get_ray_tracing_shader_group_stack_size_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetRayTracingShaderGroupStackSizeKHR").as_ptr(),
                ))
            },
            cmd_trace_rays_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdTraceRaysKHR").as_ptr()))
            },
            cmd_trace_rays_indirect_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdTraceRaysIndirectKHR").as_ptr()))
            },
            cmd_set_ray_tracing_pipeline_stack_size_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetRayTracingPipelineStackSizeKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_ray_tracing_shader_group_handles_khr`]. See
    /// [`FNGetRayTracingShaderGroupHandlesKhr`] for more information.
    pub fn get_ray_tracing_shader_group_handles_khr(&self) -> FNGetRayTracingShaderGroupHandlesKhr {
        self.get_ray_tracing_shader_group_handles_khr
    }
    ///Gets [`Self::get_ray_tracing_capture_replay_shader_group_handles_khr`]. See
    /// [`FNGetRayTracingCaptureReplayShaderGroupHandlesKhr`] for more information.
    pub fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
    ) -> FNGetRayTracingCaptureReplayShaderGroupHandlesKhr {
        self.get_ray_tracing_capture_replay_shader_group_handles_khr
    }
    ///Gets [`Self::create_ray_tracing_pipelines_khr`]. See [`FNCreateRayTracingPipelinesKhr`] for
    /// more information.
    pub fn create_ray_tracing_pipelines_khr(&self) -> FNCreateRayTracingPipelinesKhr {
        self.create_ray_tracing_pipelines_khr
    }
    ///Gets [`Self::get_ray_tracing_shader_group_stack_size_khr`]. See
    /// [`FNGetRayTracingShaderGroupStackSizeKhr`] for more information.
    pub fn get_ray_tracing_shader_group_stack_size_khr(&self) -> FNGetRayTracingShaderGroupStackSizeKhr {
        self.get_ray_tracing_shader_group_stack_size_khr
    }
    ///Gets [`Self::cmd_trace_rays_khr`]. See [`FNCmdTraceRaysKhr`] for more information.
    pub fn cmd_trace_rays_khr(&self) -> FNCmdTraceRaysKhr {
        self.cmd_trace_rays_khr
    }
    ///Gets [`Self::cmd_trace_rays_indirect_khr`]. See [`FNCmdTraceRaysIndirectKhr`] for more
    /// information.
    pub fn cmd_trace_rays_indirect_khr(&self) -> FNCmdTraceRaysIndirectKhr {
        self.cmd_trace_rays_indirect_khr
    }
    ///Gets [`Self::cmd_set_ray_tracing_pipeline_stack_size_khr`]. See
    /// [`FNCmdSetRayTracingPipelineStackSizeKhr`] for more information.
    pub fn cmd_set_ray_tracing_pipeline_stack_size_khr(&self) -> FNCmdSetRayTracingPipelineStackSizeKhr {
        self.cmd_set_ray_tracing_pipeline_stack_size_khr
    }
}
