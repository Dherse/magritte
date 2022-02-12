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
//! - A new ray tracing pipeline type with new shader domains: ray generation,
//!intersection, any-hit, closest hit, miss, and callable
//! - A shader binding indirection table to link shader groups with
//!acceleration structure items
//! - Ray tracing commands which initiate the ray pipeline traversal and
//!invocation of the various new shader domains depending on which
//!traversal conditions are metThis extension adds support for the following SPIR-V extension in
//! Vulkan:
//! - `SPV_KHR_ray_tracing`
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_KHR_spirv_1_4`]`
//! - Requires `[`VK_KHR_acceleration_structure`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_ray_tracing_pipeline]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_ray_tracing_pipeline
//!   extension>>)
//!# New functions & commands
//! - [`CmdSetRayTracingPipelineStackSizeKHR`]
//! - [`CmdTraceRaysIndirectKHR`]
//! - [`CmdTraceRaysKHR`]
//! - [`CreateRayTracingPipelinesKHR`]
//! - [`GetRayTracingCaptureReplayShaderGroupHandlesKHR`]
//! - [`GetRayTracingShaderGroupHandlesKHR`]
//! - [`GetRayTracingShaderGroupStackSizeKHR`]
//!# New structures
//! - [`RayTracingPipelineCreateInfoKHR`]
//! - [`RayTracingPipelineInterfaceCreateInfoKHR`]
//! - [`RayTracingShaderGroupCreateInfoKHR`]
//! - [`StridedDeviceAddressRegionKHR`]
//! - [`TraceRaysIndirectCommandKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceRayTracingPipelineFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//!# New enums
//! - [`RayTracingShaderGroupTypeKHR`]
//! - [`ShaderGroupShaderKHR`]
//!# New constants
//! - [`KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME`]
//! - [`KHR_RAY_TRACING_PIPELINE_SPEC_VERSION`]
//! - [`SHADER_UNUSED_KHR`]
//! - Extending [`BufferUsageFlagBits`]:
//! - `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR`
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`
//! - Extending [`PipelineBindPoint`]:
//! - `VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR`
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR`
//! - Extending [`ShaderStageFlagBits`]:
//! - `VK_SHADER_STAGE_ANY_HIT_BIT_KHR`
//! - `VK_SHADER_STAGE_CALLABLE_BIT_KHR`
//! - `VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`
//! - `VK_SHADER_STAGE_INTERSECTION_BIT_KHR`
//! - `VK_SHADER_STAGE_MISS_BIT_KHR`
//! - `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!(1) How does this extension differ from VK_NV_ray_tracing?**DISCUSSION**:The following is a
//! summary of the main functional differences between
//!VK_KHR_ray_tracing_pipeline and VK_NV_ray_tracing:
//! - added support for indirect ray tracing ([`CmdTraceRaysIndirectKHR`])
//! - uses SPV_KHR_ray_tracing instead of SPV_NV_ray_tracing
//! - refer to KHR SPIR-V enums instead of NV SPIR-V enums (which are
//!functionally equivalent and aliased to the same values).
//! - added `RayGeometryIndexKHR` built-in
//! - removed vkCompileDeferredNV compilation functionality and replaced with
//![deferred host operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations) interactions for
//!ray tracing
//! - added [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure
//! - extended [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure
//! - renamed `maxRecursionDepth` to `maxRayRecursionDepth` and it
//!has a minimum of 1 instead of 31
//! - require `shaderGroupHandleSize` to be 32 bytes
//! - added `maxRayDispatchInvocationCount`,
//!`shaderGroupHandleAlignment` and `maxRayHitAttributeSize`
//! - reworked geometry structures so they could be better shared between
//!device, host, and indirect builds
//! - changed SBT parameters to a structure and added size
//!([`StridedDeviceAddressRegionKHR`])
//! - add parameter for requesting memory requirements for host and/or device
//!build
//! - added [pipeline library](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-library)
//!   support for ray tracing
//! - added [watertightness guarantees](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-watertight)
//! - added no-null-shader pipeline flags
//!(`VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_*_SHADERS_BIT_KHR`)
//! - added [memory model interactions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-shader-call)
//!   with ray
//!tracing and define how subgroups work and can be repacked(2) Can you give a more detailed
//! comparision of differences and similarities
//!between VK_NV_ray_tracing and VK_KHR_ray_tracing_pipeline?**DISCUSSION**:The following is a more
//! detailed comparision of which commands, structures,
//!and enums are aliased, changed, or removed.
//! - Aliased functionality — enums, structures, and commands that are
//!considered equivalent:
//! - [`RayTracingShaderGroupTypeNV`] ↔
//![`RayTracingShaderGroupTypeKHR`]
//! - [`GetRayTracingShaderGroupHandlesNV`] ↔
//![`GetRayTracingShaderGroupHandlesKHR`]
//! - Changed enums, structures, and commands:
//! - [`RayTracingShaderGroupCreateInfoNV`] →
//![`RayTracingShaderGroupCreateInfoKHR`] (added
//!`pShaderGroupCaptureReplayHandle`)
//! - [`RayTracingPipelineCreateInfoNV`] →
//![`RayTracingPipelineCreateInfoKHR`] (changed type of `pGroups`,
//!added `libraries`, `pLibraryInterface`, and
//!`pDynamicState`)
//! - [`PhysicalDeviceRayTracingPropertiesNV`] →
//!VkPhysicalDeviceRayTracingPropertiesKHR (renamed `maxTriangleCount`
//!to `maxPrimitiveCount`, added
//!`shaderGroupHandleCaptureReplaySize`)
//! - [`CmdTraceRaysNV`] → [`CmdTraceRaysKHR`] (params to struct)
//! - [`CreateRayTracingPipelinesNV`] →
//![`CreateRayTracingPipelinesKHR`] (different struct, changed
//!functionality)
//! - Added enums, structures and commands:
//! - `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR``VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`,
//!`VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`,
//!`VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`,
//!`VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`,
//!`VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR` to
//![`PipelineCreateFlagBits`]
//! - [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure
//! - [`DeviceOrHostAddressKHR`] and [`DeviceOrHostAddressConstKHR`]
//!unions
//! - [`PipelineLibraryCreateInfoKHR`] struct
//! - [`RayTracingPipelineInterfaceCreateInfoKHR`] struct
//! - [`StridedDeviceAddressRegionKHR`] struct
//! - [`CmdTraceRaysIndirectKHR`] command and
//![`TraceRaysIndirectCommandKHR`] struct
//! - [`GetRayTracingCaptureReplayShaderGroupHandlesKHR`] (shader group
//!capture/replay)
//! - [`CmdSetRayTracingPipelineStackSizeKHR`] and
//![`GetRayTracingShaderGroupStackSizeKHR`] commands for stack size
//!control
//! - Functionality removed:
//! - `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
//! - [`CompileDeferredNV`] command (replaced with
//!`[`VK_KHR_deferred_host_operations`]`)(3) What are the changes between the public provisional
//! (VK_KHR_ray_tracing
//!v8) release and the internal provisional (VK_KHR_ray_tracing v9) release?
//! - Require Vulkan 1.1 and SPIR-V 1.4
//! - Added interactions with Vulkan 1.2 and
//!`[`VK_KHR_vulkan_memory_model`]`
//! - added creation time capture and replay flags
//! - added
//!`VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
//!to [`PipelineCreateFlagBits`]
//! - replace `VkStridedBufferRegionKHR` with
//![`StridedDeviceAddressRegionKHR`] and change
//![`CmdTraceRaysKHR`], [`CmdTraceRaysIndirectKHR`], to take these
//!for the shader binding table and use device addresses instead of
//!buffers.
//! - require the shader binding table buffers to have the
//!`VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR` set
//! - make `[`VK_KHR_pipeline_library`]` an interaction instead of required
//!extension
//! - rename the `libraries` member of
//![`RayTracingPipelineCreateInfoKHR`] to `pLibraryInfo` and make
//!it a pointer
//! - make `[`VK_KHR_deferred_host_operations`]` an interaction instead of
//!a required extension (later went back on this)
//! - added explicit stack size management for ray tracing pipelines
//! - removed the `maxCallableSize` member of
//![`RayTracingPipelineInterfaceCreateInfoKHR`]
//! - added the `pDynamicState` member to
//![`RayTracingPipelineCreateInfoKHR`]
//! - added `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`
//!dynamic state for ray tracing pipelines
//! - added [`GetRayTracingShaderGroupStackSizeKHR`] and
//![`CmdSetRayTracingPipelineStackSizeKHR`] commands
//! - added [`ShaderGroupShaderKHR`] enum
//! - Added `maxRayDispatchInvocationCount` limit to
//![`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Added `shaderGroupHandleAlignment` property to
//![`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Added `maxRayHitAttributeSize` property to
//![`PhysicalDeviceRayTracingPipelinePropertiesKHR`]
//! - Clarify deferred host ops for pipeline creation
//! - [`DeferredOperationKHR`] is now a top-level parameter for
//![`CreateRayTracingPipelinesKHR`]
//! - removed `VkDeferredOperationInfoKHR` structure
//! - change deferred host creation/return parameter behavior such that the
//!implementation can modify such parameters until the deferred host
//!operation completes
//! - `[`VK_KHR_deferred_host_operations`]` is required again(4) What are the changes between the
//!   internal provisional
//!(VK_KHR_ray_tracing v9) release and the final (VK_KHR_acceleration_structure
//!v11 / VK_KHR_ray_tracing_pipeline v1) release?
//! - refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation
//!flexibility and decoupling ray query support from ray pipelines:
//! - `[`VK_KHR_acceleration_structure`]` (for acceleration structure
//!operations)
//! - `[`VK_KHR_ray_tracing_pipeline`]` (for ray tracing pipeline and
//!shader stages)
//! - `[`VK_KHR_ray_query`]` (for ray queries in existing shader stages)
//! - Require `Volatile` for the following builtins in the ray generation,
//!closest hit, miss, intersection, and callable shader stages:
//! - `SubgroupSize`, `SubgroupLocalInvocationId`, `SubgroupEqMask`,
//!`SubgroupGeMask`, `SubgroupGtMask`, `SubgroupLeMask`,
//!`SubgroupLtMask`
//! - `SMIDNV`, `WarpIDNV`
//! - clarify buffer usage flags for ray tracing
//! - `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` is added as an alias
//!of `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` and is required on shader
//!binding table buffers
//! - `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` is used in
//!`[`VK_KHR_acceleration_structure`]` for `scratchData`
//! - rename `maxRecursionDepth` to `maxRayPipelineRecursionDepth`
//!(pipeline creation) and `maxRayRecursionDepth` (limit) to reduce
//!confusion
//! - Add queryable `maxRayHitAttributeSize` limit and rename members of
//![`RayTracingPipelineInterfaceCreateInfoKHR`] to
//!`maxPipelineRayPayloadSize` and `maxPipelineRayHitAttributeSize`
//!for clarity
//! - Update SPIRV capabilities to use `RayTracingKHR`
//! - extension is no longer provisional
//! - define synchronization requirements for indirect trace rays and indirect
//!buffer(5) This extension adds gl_InstanceID for the intersection, any-hit, and
//!    closest hit shaders, but in KHR_vulkan_glsl, gl_InstanceID is replaced
//!    with gl_InstanceIndex.
//!    Which should be used for Vulkan in this extension?**RESOLVED**: This extension uses
//! gl_InstanceID and maps it to `InstanceId`
//!in SPIR-V.
//!It is acknowledged that this is different than other shader stages in
//!Vulkan.
//!There are two main reasons for the difference here:
//! - symmetry with gl_PrimitiveID which is also available in these shaders
//! - there is no “baseInstance” relevant for these shaders, and so ID makes
//!it more obvious that this is zero-based.
//!# Version History
//! - Revision 1, 2020-11-12 (Mathieu Robart, Daniel Koch, Eric Werness,
//!Tobias Hector)
//! - Decomposition of the specification, from VK_KHR_ray_tracing to
//!VK_KHR_ray_tracing_pipeline (#1918,!3912)
//! - require certain subgroup and sm_shader_builtin shader builtins to be
//!decorated as volatile in the ray generation, closest hit, miss,
//!intersection, and callable stages (#1924,!3903,!3954)
//! - clarify buffer usage flags for ray tracing (#2181,!3939)
//! - rename maxRecursionDepth to maxRayPipelineRecursionDepth and
//!maxRayRecursionDepth (#2203,!3937)
//! - add queriable maxRayHitAttributeSize and rename members of
//!VkRayTracingPipelineInterfaceCreateInfoKHR (#2102,!3966)
//! - update to use `RayTracingKHR` SPIR-V capability
//! - add VUs for matching hit group type against geometry type
//!(#2245,!3994)
//! - require `RayTMaxKHR` be volatile in intersection shaders
//!(#2268,!4030)
//! - add numerical limits for ray parameters (#2235,!3960)
//! - fix SBT indexing rules for device addresses (#2308,!4079)
//! - relax formula for ray intersection candidate determination
//!(#2322,!4080)
//! - add more details on `ShaderRecordBufferKHR` variables (#2230,!4083)
//! - clarify valid bits for `InstanceCustomIndexKHR`
//!(GLSL/GLSL#19,!4128)
//! - allow at most one `IncomingRayPayloadKHR`,
//!`IncomingCallableDataKHR`, and `HitAttributeKHR` (!4129)
//! - add minimum for maxShaderGroupStride (#2353,!4131)
//! - require VK_KHR_pipeline_library extension to be supported
//!(#2348,!4135)
//! - clarify meaning of 'geometry index' (#2272,!4137)
//! - restrict traces to TLAS (#2239,!4141)
//! - add note about maxPipelineRayPayloadSize (#2383,!4172)
//! - do not require raygen shader in pipeline libraries (!4185)
//! - define sync for indirect trace rays and indirect buffer (#2407,!4208)
//!# Other info
//! * 2020-11-12
//!*
//! - This extension requires
//![`SPV_KHR_ray_tracing`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_ray_tracing.html)
//! - This extension provides API support for
//![`GLSL_EXT_ray_tracing`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_ray_tracing.txt)
//! - This extension interacts with [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2)
//!   and
//!`[`VK_KHR_vulkan_memory_model`]`, adding the [shader-call-related](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-call-related) relation of invocations, [shader-call-order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-call-order) partial order of dynamic instances of instructions,
//!and the [`ShaderCallKHR`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-shadercall) scope.
//! - This extension interacts with `[`VK_KHR_pipeline_library`]`, enabling
//!pipeline libraries to be used with ray tracing pipelines and enabling
//!usage of [`RayTracingPipelineInterfaceCreateInfoKHR`].
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
//! - [`CmdSetRayTracingPipelineStackSizeKHR`]
//! - [`CmdTraceRaysIndirectKHR`]
//! - [`CmdTraceRaysKHR`]
//! - [`CreateRayTracingPipelinesKHR`]
//! - [`GetRayTracingCaptureReplayShaderGroupHandlesKHR`]
//! - [`GetRayTracingShaderGroupHandlesKHR`]
//! - [`GetRayTracingShaderGroupStackSizeKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///[VK_SHADER_UNUSED_KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html) - Sentinel for an unused shader index
///# C Specifications
///[`SHADER_UNUSED_KHR`] is a special shader index used to indicate that a
///ray generation, miss, or callable shader member is not used.
///```c
///#define VK_SHADER_UNUSED_KHR              (~0U)
///```
///or the equivalent
///```c
///#define VK_SHADER_UNUSED_NV               VK_SHADER_UNUSED_KHR
///```
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`VK_NV_ray_tracing`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = !0;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_tracing_pipeline");
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
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_GENERAL`] indicates a shader
///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP`] specifies
///a shader group that only hits triangles and **must** not contain an
///intersection shader, only closest hit and any-hit shaders.
/// - [`RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP`]
///specifies a shader group that only intersects with custom geometry and
///**must** contain an intersection shader and **may** contain closest hit and
///any-hit shaders.
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`VK_NV_ray_tracing`]
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
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl const Default for RayTracingShaderGroupTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("RayTracingShaderGroupTypeKHR")
            .field(match *self {
                Self::RAY_TRACING_SHADER_GROUP_TYPE_GENERAL => &"RAY_TRACING_SHADER_GROUP_TYPE_GENERAL",
                Self::RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP => {
                    &"RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP"
                },
                Self::RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP => {
                    &"RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP"
                },
                other => unreachable!("invalid value for `RayTracingShaderGroupTypeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl RayTracingShaderGroupTypeKHR {
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_GENERAL`] indicates a shader
    ///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
    ///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
    ///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL: Self = Self(0);
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP`] specifies
    ///a shader group that only hits triangles and **must** not contain an
    ///intersection shader, only closest hit and any-hit shaders.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP: Self = Self(1);
    ///[`RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP`]
    ///specifies a shader group that only intersects with custom geometry and
    ///**must** contain an intersection shader and **may** contain closest hit and
    ///any-hit shaders.
    pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV: Self = Self::RAY_TRACING_SHADER_GROUP_TYPE_GENERAL;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV: Self =
        Self::RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV: Self =
        Self::RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP;
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
}
///[VkShaderGroupShaderKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html) - Shader group shaders
///# C Specifications
///Possible values of `groupShader` in
///[`GetRayTracingShaderGroupStackSizeKHR`] are:
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
/// - [`SHADER_GROUP_SHADER_GENERAL`] uses the shader specified in
///the group with
///[`RayTracingShaderGroupCreateInfoKHR::general_shader`]
/// - [`SHADER_GROUP_SHADER_CLOSEST_HIT`] uses the shader specified
///in the group with
///[`RayTracingShaderGroupCreateInfoKHR::closest_hit_shader`]
/// - [`SHADER_GROUP_SHADER_ANY_HIT`] uses the shader specified in
///the group with
///[`RayTracingShaderGroupCreateInfoKHR::any_hit_shader`]
/// - [`SHADER_GROUP_SHADER_INTERSECTION`] uses the shader specified
///in the group with
///[`RayTracingShaderGroupCreateInfoKHR::intersection_shader`]
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`GetRayTracingShaderGroupStackSizeKHR`]
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
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderGroupShaderKHR(i32);
impl const Default for ShaderGroupShaderKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ShaderGroupShaderKHR")
            .field(match *self {
                Self::SHADER_GROUP_SHADER_GENERAL => &"SHADER_GROUP_SHADER_GENERAL",
                Self::SHADER_GROUP_SHADER_CLOSEST_HIT => &"SHADER_GROUP_SHADER_CLOSEST_HIT",
                Self::SHADER_GROUP_SHADER_ANY_HIT => &"SHADER_GROUP_SHADER_ANY_HIT",
                Self::SHADER_GROUP_SHADER_INTERSECTION => &"SHADER_GROUP_SHADER_INTERSECTION",
                other => unreachable!("invalid value for `ShaderGroupShaderKHR`: {:?}", other),
            })
            .finish()
    }
}
impl ShaderGroupShaderKHR {
    ///[`SHADER_GROUP_SHADER_GENERAL`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`generalShader`
    pub const SHADER_GROUP_SHADER_GENERAL: Self = Self(0);
    ///[`SHADER_GROUP_SHADER_CLOSEST_HIT`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`closestHitShader`
    pub const SHADER_GROUP_SHADER_CLOSEST_HIT: Self = Self(1);
    ///[`SHADER_GROUP_SHADER_ANY_HIT`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`anyHitShader`
    pub const SHADER_GROUP_SHADER_ANY_HIT: Self = Self(2);
    ///[`SHADER_GROUP_SHADER_INTERSECTION`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`intersectionShader`
    pub const SHADER_GROUP_SHADER_INTERSECTION: Self = Self(3);
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
}
