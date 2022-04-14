//![VK_NV_ray_tracing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_ray_tracing.html) - device extension
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
//! - Acceleration structure objects and build commands
//! - A new pipeline type with new shader domains
//! - An indirection table to link shader groups with acceleration structure items
//!This extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_NV_ray_tracing`
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`khr_get_memory_requirements2`]`
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_ray_tracing]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the VK_NV_ray_tracing
//!   extension>>)
//!# New handles
//! - [`AccelerationStructureNV`]
//!# New functions & commands
//! - [`bind_acceleration_structure_memory_nv`]
//! - [`cmd_build_acceleration_structure_nv`]
//! - [`cmd_copy_acceleration_structure_nv`]
//! - [`cmd_trace_rays_nv`]
//! - [`cmd_write_acceleration_structures_properties_nv`]
//! - [`compile_deferred_nv`]
//! - [`create_acceleration_structure_nv`]
//! - [`create_ray_tracing_pipelines_nv`]
//! - [`destroy_acceleration_structure_nv`]
//! - [`get_acceleration_structure_handle_nv`]
//! - [`get_acceleration_structure_memory_requirements_nv`]
//! - [`get_ray_tracing_shader_group_handles_nv`]
//!# New structures
//! - [`AabbPositionsNV`]
//! - [`AccelerationStructureCreateInfoNV`]
//! - [`AccelerationStructureInfoNV`]
//! - [`AccelerationStructureInstanceNV`]
//! - [`AccelerationStructureMemoryRequirementsInfoNV`]
//! - [`BindAccelerationStructureMemoryInfoNV`]
//! - [`GeometryAabbNV`]
//! - [`GeometryDataNV`]
//! - [`GeometryNV`]
//! - [`GeometryTrianglesNV`]
//! - [`MemoryRequirements2KHR`]
//! - [`RayTracingPipelineCreateInfoNV`]
//! - [`RayTracingShaderGroupCreateInfoNV`]
//! - [`TransformMatrixNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceRayTracingPropertiesNV`]
//! - Extending [`WriteDescriptorSet`]:  - [`WriteDescriptorSetAccelerationStructureNV`]
//!# New enums
//! - [`AccelerationStructureMemoryRequirementsTypeNV`]
//! - [`AccelerationStructureTypeNV`]
//! - [`BuildAccelerationStructureFlagBitsNV`]
//! - [`CopyAccelerationStructureModeNV`]
//! - [`GeometryFlagBitsNV`]
//! - [`GeometryInstanceFlagBitsNV`]
//! - [`GeometryTypeNV`]
//! - [`RayTracingShaderGroupTypeNV`]
//!# New bitmasks
//! - [`BuildAccelerationStructureFlagsNV`]
//! - [`GeometryFlagsNV`]
//! - [`GeometryInstanceFlagsNV`]
//!# New constants
//! - [`NV_RAY_TRACING_EXTENSION_NAME`]
//! - [`NV_RAY_TRACING_SPEC_VERSION`]
//! - [`SHADER_UNUSED_NV`]
//! - Extending [`AccelerationStructureTypeKHR`]:  -
//!   `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV`  -
//!   `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV`
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV`  -
//!   `VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV`
//! - Extending [`BuildAccelerationStructureFlagBitsKHR`]:  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV`  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV`  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV`  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV`  -
//!   `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV`
//! - Extending [`CopyAccelerationStructureModeKHR`]:  -
//!   `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV`  -
//!   `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV`
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT`
//! - Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`
//! - Extending [`GeometryFlagBitsKHR`]:  - `VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV`  -
//!   `VK_GEOMETRY_OPAQUE_BIT_NV`
//! - Extending [`GeometryInstanceFlagBitsKHR`]:  - `VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV`  -
//!   `VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV`  -
//!   `VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV`  -
//!   `VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV`
//! - Extending [`GeometryTypeKHR`]:  - `VK_GEOMETRY_TYPE_AABBS_NV`  -
//!   `VK_GEOMETRY_TYPE_TRIANGLES_NV`
//! - Extending [`IndexType`]:  - `VK_INDEX_TYPE_NONE_NV`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV`
//! - Extending [`PipelineBindPoint`]:  - `VK_PIPELINE_BIND_POINT_RAY_TRACING_NV`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
//! - Extending [`PipelineStageFlagBits`]:  -
//!   `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
//! - Extending [`RayTracingShaderGroupTypeKHR`]:  - `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`
//!   - `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`  -
//!   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV`
//! - Extending [`ShaderStageFlagBits`]:  - `VK_SHADER_STAGE_ANY_HIT_BIT_NV`  -
//!   `VK_SHADER_STAGE_CALLABLE_BIT_NV`  - `VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`  -
//!   `VK_SHADER_STAGE_INTERSECTION_BIT_NV`  - `VK_SHADER_STAGE_MISS_BIT_NV`  -
//!   `VK_SHADER_STAGE_RAYGEN_BIT_NV`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`  - `VK_STRUCTURE_TYPE_GEOMETRY_NV`  -
//!   `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`  -
//!   `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
//!# Known issues & F.A.Q
//!1) Are there issues? **RESOLVED** : Yes.
//!# Version History
//! - Revision 1, 2018-09-11 (Robert Stepinski, Nuno Subtil, Eric Werness)  - Internal revisions
//! - Revision 2, 2018-10-19 (Eric Werness)  - rename to VK_NV_ray_tracing, add support for
//!   callables.  - too many updates to list
//! - Revision 3, 2018-11-20 (Daniel Koch)  - update to use InstanceId instead of InstanceIndex as
//!   implemented.
//!# Other info
//! * 2018-11-20
//! * - This extension requires [`SPV_NV_ray_tracing`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_ray_tracing.html)
//!   - This extension provides API support for [`GL_NV_ray_tracing`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_ray_tracing.txt)
//! * - Eric Werness, NVIDIA  - Ashwin Lele, NVIDIA  - Robert Stepinski, NVIDIA  - Nuno Subtil,
//!   NVIDIA  - Christoph Kubisch, NVIDIA  - Martin Stich, NVIDIA  - Daniel Koch, NVIDIA  - Jeff
//!   Bolz, NVIDIA  - Joshua Barczak, Intel  - Tobias Hector, AMD  - Henrik Rydgard, NVIDIA  -
//!   Pascal Gautron, NVIDIA
//!# Related
//! - [`SHADER_UNUSED_NV`]
//! - [`AabbPositionsNV`]
//! - [`AccelerationStructureCreateInfoNV`]
//! - [`AccelerationStructureInfoNV`]
//! - [`AccelerationStructureInstanceNV`]
//! - [`AccelerationStructureMemoryRequirementsInfoNV`]
//! - [`AccelerationStructureMemoryRequirementsTypeNV`]
//! - [`AccelerationStructureNV`]
//! - [`AccelerationStructureTypeNV`]
//! - [`BindAccelerationStructureMemoryInfoNV`]
//! - [`BuildAccelerationStructureFlagBitsNV`]
//! - [`BuildAccelerationStructureFlagsNV`]
//! - [`CopyAccelerationStructureModeNV`]
//! - [`GeometryAabbNV`]
//! - [`GeometryDataNV`]
//! - [`GeometryFlagBitsNV`]
//! - [`GeometryFlagsNV`]
//! - [`GeometryInstanceFlagBitsNV`]
//! - [`GeometryInstanceFlagsNV`]
//! - [`GeometryNV`]
//! - [`GeometryTrianglesNV`]
//! - [`GeometryTypeNV`]
//! - [`MemoryRequirements2KHR`]
//! - [`PhysicalDeviceRayTracingPropertiesNV`]
//! - [`RayTracingPipelineCreateInfoNV`]
//! - [`RayTracingShaderGroupCreateInfoNV`]
//! - [`RayTracingShaderGroupTypeNV`]
//! - [`TransformMatrixNV`]
//! - [`WriteDescriptorSetAccelerationStructureNV`]
//! - [`bind_acceleration_structure_memory_nv`]
//! - [`cmd_build_acceleration_structure_nv`]
//! - [`cmd_copy_acceleration_structure_nv`]
//! - [`cmd_trace_rays_nv`]
//! - [`cmd_write_acceleration_structures_properties_nv`]
//! - [`compile_deferred_nv`]
//! - [`create_acceleration_structure_nv`]
//! - [`create_ray_tracing_pipelines_nv`]
//! - [`destroy_acceleration_structure_nv`]
//! - [`get_acceleration_structure_handle_nv`]
//! - [`get_acceleration_structure_memory_requirements_nv`]
//! - [`get_ray_tracing_shader_group_handles_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    extensions::{
        khr_acceleration_structure::{
            AccelerationStructureTypeKHR, BuildAccelerationStructureFlagsKHR, CopyAccelerationStructureModeKHR,
            GeometryFlagsKHR, GeometryTypeKHR,
        },
        khr_ray_tracing_pipeline::{FNGetRayTracingShaderGroupHandlesKhr, RayTracingShaderGroupTypeKHR},
    },
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceMemory,
        DeviceSize, Format, IndexType, Instance, PhysicalDevice, Pipeline, PipelineCache, PipelineCreateFlags,
        PipelineLayout, PipelineShaderStageCreateInfo, QueryPool, QueryType, StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
    vulkan1_3::PipelineCreationFeedbackCreateInfo,
    AsRaw, Handle, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_ray_tracing");
///[vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html) - Deferred compilation of shaders
///# C Specifications
///To compile a deferred shader in a pipeline call:
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkCompileDeferredNV(
///    VkDevice                                    device,
///    VkPipeline                                  pipeline,
///    uint32_t                                    shader);
///```
///# Parameters
/// - [`device`] is the logical device containing the ray tracing pipeline.
/// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
/// - [`shader`] is the index of the shader to compile.
///# Description
///## Valid Usage
/// - [`pipeline`] **must**  be a ray tracing pipeline
/// - [`pipeline`] **must**  have been created with `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
/// - [`shader`] **must**  not have been called as a deferred compile before
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
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
#[doc(alias = "vkCompileDeferredNV")]
pub type FNCompileDeferredNv =
    Option<unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> VulkanResultCodes>;
///[vkCreateAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html) - Create a new acceleration structure object
///# C Specifications
///To create acceleration structures, call:
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkCreateAccelerationStructureNV(
///    VkDevice                                    device,
///    const VkAccelerationStructureCreateInfoNV*  pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkAccelerationStructureNV*                  pAccelerationStructure);
///```
///# Parameters
/// - [`device`] is the logical device that creates the buffer object.
/// - [`p_create_info`] is a pointer to a [`AccelerationStructureCreateInfoNV`] structure containing
///   parameters affecting creation of the acceleration structure.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_acceleration_structure`] is a pointer to a [`AccelerationStructureNV`] handle in which the
///   resulting acceleration structure object is returned.
///# Description
///Similarly to other objects in Vulkan, the acceleration structure creation
///merely creates an object with a specific “shape” as specified by the
///information in [`AccelerationStructureInfoNV`] and `compactedSize`
///in [`p_create_info`].
///Populating the data in the object after allocating and binding memory is
///done with [`cmd_build_acceleration_structure_nv`] and
///[`cmd_copy_acceleration_structure_nv`].Acceleration structure creation uses the count and type
/// information from the
///geometries, but does not use the data references in the structures.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`AccelerationStructureCreateInfoNV`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_acceleration_structure`] **must**  be a valid pointer to a [`AccelerationStructureNV`]
///   handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureCreateInfoNV`]
/// - [`AccelerationStructureNV`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateAccelerationStructureNV")]
pub type FNCreateAccelerationStructureNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoNV<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> VulkanResultCodes,
>;
///[vkDestroyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html) - Destroy an acceleration structure object
///# C Specifications
///To destroy an acceleration structure, call:
///```c
///// Provided by VK_NV_ray_tracing
///void vkDestroyAccelerationStructureNV(
///    VkDevice                                    device,
///    VkAccelerationStructureNV                   accelerationStructure,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the logical device that destroys the buffer.
/// - [`acceleration_structure`] is the acceleration structure to destroy.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
///# Description
///## Valid Usage
/// - All submitted commands that refer to [`acceleration_structure`] **must**  have completed
///   execution
/// - If [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created, a
///   compatible set of callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created,
///   [`p_allocator`] **must**  be `NULL`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`acceleration_structure`] is not [`crate::Handle::null`], [`acceleration_structure`]
///   **must**  be a valid [`AccelerationStructureNV`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`acceleration_structure`] is a valid handle, it  **must**  have been created, allocated,
///   or retrieved from [`device`]
///
///## Host Synchronization
/// - Host access to [`acceleration_structure`] **must**  be externally synchronized
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyAccelerationStructureNV")]
pub type FNDestroyAccelerationStructureNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetAccelerationStructureMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) - Get acceleration structure memory requirements
///# C Specifications
///An acceleration structure has memory requirements for the structure object
///itself, scratch space for the build, and scratch space for the update.Scratch space is allocated
/// as a [`Buffer`], so for
///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
///and
///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV`
///the `pMemoryRequirements->alignment` and
///`pMemoryRequirements->memoryTypeBits` values returned by this call  **must**
///be filled with zero, and  **should**  be ignored by the application.To query the memory
/// requirements, call:
///```c
///// Provided by VK_NV_ray_tracing
///void vkGetAccelerationStructureMemoryRequirementsNV(
///    VkDevice                                    device,
///    const VkAccelerationStructureMemoryRequirementsInfoNV* pInfo,
///    VkMemoryRequirements2KHR*                   pMemoryRequirements);
///```
///# Parameters
/// - [`device`] is the logical device on which the acceleration structure was created.
/// - [`p_info`] is a pointer to a [`AccelerationStructureMemoryRequirementsInfoNV`] structure
///   specifying the acceleration structure to get memory requirements for.
/// - [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2KHR`] structure in which the
///   requested acceleration structure memory requirements are returned.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid
///   [`AccelerationStructureMemoryRequirementsInfoNV`] structure
/// - [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2KHR`]
///   structure
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsInfoNV`]
/// - [`Device`]
/// - [`MemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
pub type FNGetAccelerationStructureMemoryRequirementsNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'lt>,
        p_memory_requirements: *mut MemoryRequirements2<'lt>,
    ),
>;
///[vkBindAccelerationStructureMemoryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) - Bind acceleration structure memory
///# C Specifications
///To attach memory to one or more acceleration structures at a time, call:
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkBindAccelerationStructureMemoryNV(
///    VkDevice                                    device,
///    uint32_t                                    bindInfoCount,
///    const VkBindAccelerationStructureMemoryInfoNV* pBindInfos);
///```
///# Parameters
/// - [`device`] is the logical device that owns the acceleration structures and memory.
/// - [`bind_info_count`] is the number of elements in [`p_bind_infos`].
/// - [`p_bind_infos`] is a pointer to an array of [`BindAccelerationStructureMemoryInfoNV`]
///   structures describing acceleration structures and memory to bind.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_bind_infos`] **must**  be a valid pointer to an array of [`bind_info_count`] valid
///   [`BindAccelerationStructureMemoryInfoNV`] structures
/// - [`bind_info_count`] **must**  be greater than `0`
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`nv_ray_tracing`]
/// - [`BindAccelerationStructureMemoryInfoNV`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
pub type FNBindAccelerationStructureMemoryNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html) - Get opaque acceleration structure handle
///# C Specifications
///To allow constructing geometry instances with device code if desired, we
///need to be able to query a opaque handle for an acceleration structure.
///This handle is a value of 8 bytes.
///To get this handle, call:
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkGetAccelerationStructureHandleNV(
///    VkDevice                                    device,
///    VkAccelerationStructureNV                   accelerationStructure,
///    size_t                                      dataSize,
///    void*                                       pData);
///```
///# Parameters
/// - [`device`] is the logical device that owns the acceleration structures.
/// - [`acceleration_structure`] is the acceleration structure.
/// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
/// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
///# Description
///## Valid Usage
/// - [`data_size`] **must**  be large enough to contain the result of the query, as described above
/// - [`acceleration_structure`] **must**  be bound completely and contiguously to a single
///   [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
/// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`] **must**  be greater than `0`
/// - [`acceleration_structure`] **must**  have been created, allocated, or retrieved from
///   [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
pub type FNGetAccelerationStructureHandleNv = Option<
    unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResultCodes,
>;
///[vkCreateRayTracingPipelinesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html) - Creates a new ray tracing pipeline object
///# C Specifications
///To create ray tracing pipelines, call:
///```c
///// Provided by VK_NV_ray_tracing
///VkResult vkCreateRayTracingPipelinesNV(
///    VkDevice                                    device,
///    VkPipelineCache                             pipelineCache,
///    uint32_t                                    createInfoCount,
///    const VkRayTracingPipelineCreateInfoNV*     pCreateInfos,
///    const VkAllocationCallbacks*                pAllocator,
///    VkPipeline*                                 pPipelines);
///```
///# Parameters
/// - [`device`] is the logical device that creates the ray tracing pipelines.
/// - [`pipeline_cache`] is either [`crate::Handle::null`], indicating that pipeline caching is disabled, or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache) object, in which case use of that cache is enabled for the duration of the command.
/// - [`create_info_count`] is the length of the [`p_create_infos`] and [`p_pipelines`] arrays.
/// - [`p_create_infos`] is a pointer to an array of [`RayTracingPipelineCreateInfoNV`] structures.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_pipelines`] is a pointer to an array in which the resulting ray tracing pipeline objects
///   are returned.
///# Description
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
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`pipeline_cache`] is not [`crate::Handle::null`], [`pipeline_cache`] **must**  be a valid
///   [`PipelineCache`] handle
/// - [`p_create_infos`] **must**  be a valid pointer to an array of [`create_info_count`] valid
///   [`RayTracingPipelineCreateInfoNV`] structures
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_pipelines`] **must**  be a valid pointer to an array of [`create_info_count`][`Pipeline`]
///   handles
/// - [`create_info_count`] **must**  be greater than `0`
/// - If [`pipeline_cache`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_INVALID_SHADER_NV`
///# Related
/// - [`nv_ray_tracing`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`Pipeline`]
/// - [`PipelineCache`]
/// - [`RayTracingPipelineCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
pub type FNCreateRayTracingPipelinesNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_pipelines: *mut Pipeline,
    ) -> VulkanResultCodes,
>;
///[vkCmdCopyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) - Copy an acceleration structure
///# C Specifications
///To copy an acceleration structure call:
///```c
///// Provided by VK_NV_ray_tracing
///void vkCmdCopyAccelerationStructureNV(
///    VkCommandBuffer                             commandBuffer,
///    VkAccelerationStructureNV                   dst,
///    VkAccelerationStructureNV                   src,
///    VkCopyAccelerationStructureModeKHR          mode);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`dst`] is the target acceleration structure for the copy.
/// - [`src`] is the source acceleration structure for the copy.
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///Accesses to [`src`] and [`dst`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
///`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR` as appropriate.
///## Valid Usage
/// - [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or
///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
/// - The source acceleration structure [`src`] **must**  have been constructed prior to the
///   execution of this command
/// - If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`] **must**  have been
///   constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the build
/// - The `buffer` used to create [`src`] **must**  be bound to device memory
/// - The `buffer` used to create [`dst`] **must**  be bound to device memory
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
/// - [`src`] **must**  be a valid [`AccelerationStructureNV`] handle
/// - [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - Each of [`command_buffer`], [`dst`], and [`src`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`CommandBuffer`]
/// - [`CopyAccelerationStructureModeKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
pub type FNCmdCopyAccelerationStructureNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ),
>;
///[vkCmdWriteAccelerationStructuresPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) - Write acceleration structure result parameters to query results.
///# C Specifications
///To query acceleration structure size parameters call:
///```c
///// Provided by VK_NV_ray_tracing
///void vkCmdWriteAccelerationStructuresPropertiesNV(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    accelerationStructureCount,
///    const VkAccelerationStructureNV*            pAccelerationStructures,
///    VkQueryType                                 queryType,
///    VkQueryPool                                 queryPool,
///    uint32_t                                    firstQuery);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`acceleration_structure_count`] is the count of acceleration structures for which to query
///   the property.
/// - [`p_acceleration_structures`] is a pointer to an array of existing previously built
///   acceleration structures.
/// - [`query_type`] is a [`QueryType`] value specifying the type of queries managed by the pool.
/// - [`query_pool`] is the query pool that will manage the results of the query.
/// - [`first_query`] is the first query index within the query pool that will contain the
///   [`acceleration_structure_count`] number of results.
///# Description
///Accesses to any of the acceleration structures listed in
///[`p_acceleration_structures`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`.
///## Valid Usage
/// - [`query_pool`] **must**  have been created with a [`query_type`] matching [`query_type`]
/// - The queries identified by [`query_pool`] and [`first_query`] **must**  be *unavailable*
/// - `accelerationStructure` **must**  be bound completely and contiguously to a single
///   [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]
/// - All acceleration structures in [`p_acceleration_structures`] **must**  have been built prior
///   to the execution of this command
/// - All acceleration structures in [`p_acceleration_structures`] **must**  have been built with
///   `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` if [`query_type`] is
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
/// - [`query_type`] **must**  be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_acceleration_structures`] **must**  be a valid pointer to an array of
///   [`acceleration_structure_count`] valid [`AccelerationStructureNV`] handles
/// - [`query_type`] **must**  be a valid [`QueryType`] value
/// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - [`acceleration_structure_count`] **must**  be greater than `0`
/// - Each of [`command_buffer`], [`query_pool`], and the elements of [`p_acceleration_structures`]
///   **must**  have been created, allocated, or retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`CommandBuffer`]
/// - [`QueryPool`]
/// - [`QueryType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
pub type FNCmdWriteAccelerationStructuresPropertiesNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ),
>;
///[vkCmdBuildAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) - Build an acceleration structure
///# C Specifications
///To build an acceleration structure call:
///```c
///// Provided by VK_NV_ray_tracing
///void vkCmdBuildAccelerationStructureNV(
///    VkCommandBuffer                             commandBuffer,
///    const VkAccelerationStructureInfoNV*        pInfo,
///    VkBuffer                                    instanceData,
///    VkDeviceSize                                instanceOffset,
///    VkBool32                                    update,
///    VkAccelerationStructureNV                   dst,
///    VkAccelerationStructureNV                   src,
///    VkBuffer                                    scratch,
///    VkDeviceSize                                scratchOffset);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`p_info`] contains the shared information for the acceleration structure’s structure.
/// - [`instance_data`] is the buffer containing an array of [`AccelerationStructureInstanceKHR`]
///   structures defining acceleration structures. This parameter  **must**  be `NULL` for bottom
///   level acceleration structures.
/// - [`instance_offset`] is the offset in bytes (relative to the start of [`instance_data`]) at
///   which the instance data is located.
/// - [`update`] specifies whether to update the [`dst`] acceleration structure with the data in
///   [`src`].
/// - [`dst`] is a pointer to the target acceleration structure for the build.
/// - [`src`] is a pointer to an existing acceleration structure that is to be used to update the
///   [`dst`] acceleration structure.
/// - [`scratch`] is the [`Buffer`] that will be used as scratch memory for the build.
/// - [`scratch_offset`] is the offset in bytes relative to the start of [`scratch`] that will be
///   used as a scratch memory.
///# Description
///Accesses to [`dst`], [`src`], and [`scratch`] **must**  be
///[synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
///`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`.
///## Valid Usage
/// - `geometryCount` **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
/// - [`dst`] **must**  have been created with compatible [`AccelerationStructureInfoNV`] where
///   [`AccelerationStructureInfoNV::type_`] and [`AccelerationStructureInfoNV::flags`] are
///   identical, [`AccelerationStructureInfoNV::instance_count`] and
///   [`AccelerationStructureInfoNV::geometry_count`] for [`dst`] are greater than or equal to the
///   build size and each geometry in [`AccelerationStructureInfoNV::geometries`] for [`dst`] has
///   greater than or equal to the number of vertices, indices, and AABBs
/// - If [`update`] is [`TRUE`], [`src`] **must**  not be [`crate::Handle::null`]
/// - If [`update`] is [`TRUE`], [`src`] **must**  have previously been constructed with
///   `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV` set in
///   [`AccelerationStructureInfoNV::flags`] in the original build
/// - If [`update`] is [`FALSE`], the `size` member of the [`MemoryRequirements`] structure returned
///   from a call to [`get_acceleration_structure_memory_requirements_nv`] with
///   [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`] and
///   [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV` **must**  be less than
///   or equal to the size of [`scratch`] minus [`scratch_offset`]
/// - If [`update`] is [`TRUE`], the `size` member of the [`MemoryRequirements`] structure returned
///   from a call to [`get_acceleration_structure_memory_requirements_nv`] with
///   [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`] and
///   [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV` **must**  be less than
///   or equal to the size of [`scratch`] minus [`scratch_offset`]
/// - [`scratch`] **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
/// - If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  have been
///   created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
/// - Each [`AccelerationStructureInstanceKHR::acceleration_structure_reference`] value in
///   [`instance_data`] **must**  be a valid device address containing a value obtained from
///   [`get_acceleration_structure_handle_nv`]
/// -    If [`update`] is [`TRUE`], then objects that were previously active  **must**  not be made inactive as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
/// -    If [`update`] is [`TRUE`], then objects that were previously inactive  **must**  not be made active as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
/// -    If [`update`] is [`TRUE`], the [`src`] and [`dst`] objects  **must**  either be the same object or not have any [memory aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing)
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid [`AccelerationStructureInfoNV`] structure
/// - If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  be a valid
///   [`Buffer`] handle
/// - [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
/// - If [`src`] is not [`crate::Handle::null`], [`src`] **must**  be a valid
///   [`AccelerationStructureNV`] handle
/// - [`scratch`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - Each of [`command_buffer`], [`dst`], [`instance_data`], [`scratch`], and [`src`] that are
///   valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved
///   from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureInfoNV`]
/// - [`AccelerationStructureNV`]
/// - [`Bool32`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
pub type FNCmdBuildAccelerationStructureNv = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureInfoNV<'lt>,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ),
>;
///[vkCmdTraceRaysNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html) - Initialize a ray tracing dispatch
///# C Specifications
///To dispatch ray tracing use:
///```c
///// Provided by VK_NV_ray_tracing
///void vkCmdTraceRaysNV(
///    VkCommandBuffer                             commandBuffer,
///    VkBuffer                                    raygenShaderBindingTableBuffer,
///    VkDeviceSize                                raygenShaderBindingOffset,
///    VkBuffer                                    missShaderBindingTableBuffer,
///    VkDeviceSize                                missShaderBindingOffset,
///    VkDeviceSize                                missShaderBindingStride,
///    VkBuffer                                    hitShaderBindingTableBuffer,
///    VkDeviceSize                                hitShaderBindingOffset,
///    VkDeviceSize                                hitShaderBindingStride,
///    VkBuffer                                    callableShaderBindingTableBuffer,
///    VkDeviceSize                                callableShaderBindingOffset,
///    VkDeviceSize                                callableShaderBindingStride,
///    uint32_t                                    width,
///    uint32_t                                    height,
///    uint32_t                                    depth);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`raygen_shader_binding_table_buffer`] is the buffer object that holds the shader binding
///   table data for the ray generation shader stage.
/// - [`raygen_shader_binding_offset`] is the offset in bytes (relative to
///   [`raygen_shader_binding_table_buffer`]) of the ray generation shader being used for the trace.
/// - [`miss_shader_binding_table_buffer`] is the buffer object that holds the shader binding table
///   data for the miss shader stage.
/// - [`miss_shader_binding_offset`] is the offset in bytes (relative to
///   [`miss_shader_binding_table_buffer`]) of the miss shader being used for the trace.
/// - [`miss_shader_binding_stride`] is the size in bytes of each shader binding table record in
///   [`miss_shader_binding_table_buffer`].
/// - [`hit_shader_binding_table_buffer`] is the buffer object that holds the shader binding table
///   data for the hit shader stages.
/// - [`hit_shader_binding_offset`] is the offset in bytes (relative to
///   [`hit_shader_binding_table_buffer`]) of the hit shader group being used for the trace.
/// - [`hit_shader_binding_stride`] is the size in bytes of each shader binding table record in
///   [`hit_shader_binding_table_buffer`].
/// - [`callable_shader_binding_table_buffer`] is the buffer object that holds the shader binding
///   table data for the callable shader stage.
/// - [`callable_shader_binding_offset`] is the offset in bytes (relative to
///   [`callable_shader_binding_table_buffer`]) of the callable shader being used for the trace.
/// - [`callable_shader_binding_stride`] is the size in bytes of each shader binding table record in
///   [`callable_shader_binding_table_buffer`].
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
/// - [`command_buffer`] **must**  not be a protected command buffer
/// -    This command  **must**  not cause a pipeline trace ray instruction to be executed from a shader invocation with a [recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) greater than the value of `maxRecursionDepth` used to create the bound ray tracing pipeline
/// - If [`raygen_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely
///   and contiguously to a single [`DeviceMemory`] object
/// - [`raygen_shader_binding_offset`] **must**  be less than the size of
///   [`raygen_shader_binding_table_buffer`]
/// - [`raygen_shader_binding_offset`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
/// - If [`miss_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely
///   and contiguously to a single [`DeviceMemory`] object
/// - [`miss_shader_binding_offset`] **must**  be less than the size of
///   [`miss_shader_binding_table_buffer`]
/// - [`miss_shader_binding_offset`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
/// - If [`hit_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely
///   and contiguously to a single [`DeviceMemory`] object
/// - [`hit_shader_binding_offset`] **must**  be less than the size of
///   [`hit_shader_binding_table_buffer`]
/// - [`hit_shader_binding_offset`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
/// - If [`callable_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound
///   completely and contiguously to a single [`DeviceMemory`] object
/// - [`callable_shader_binding_offset`] **must**  be less than the size of
///   [`callable_shader_binding_table_buffer`]
/// - [`callable_shader_binding_offset`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
/// - [`miss_shader_binding_stride`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
/// - [`hit_shader_binding_stride`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
/// - [`callable_shader_binding_stride`] **must**  be a multiple of
///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
/// - [`miss_shader_binding_stride`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
/// - [`hit_shader_binding_stride`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
/// - [`callable_shader_binding_stride`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
/// - [`width`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0]
/// - [`height`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1]
/// - [`depth`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`raygen_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
/// - If [`miss_shader_binding_table_buffer`] is not [`crate::Handle::null`],
///   [`miss_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
/// - If [`hit_shader_binding_table_buffer`] is not [`crate::Handle::null`],
///   [`hit_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
/// - If [`callable_shader_binding_table_buffer`] is not [`crate::Handle::null`],
///   [`callable_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
///   operations
/// - This command  **must**  only be called outside of a render pass instance
/// - Each of [`callable_shader_binding_table_buffer`], [`command_buffer`],
///   [`hit_shader_binding_table_buffer`], [`miss_shader_binding_table_buffer`], and
///   [`raygen_shader_binding_table_buffer`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`nv_ray_tracing`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdTraceRaysNV")]
pub type FNCmdTraceRaysNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ),
>;
///[VkAccelerationStructureMemoryRequirementsTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html) - Acceleration structure memory requirement type
///# C Specifications
///Possible values of `type` in
///[`AccelerationStructureMemoryRequirementsInfoNV`] are:,
///```c
///// Provided by VK_NV_ray_tracing
///typedef enum VkAccelerationStructureMemoryRequirementsTypeNV {
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV = 0,
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV = 1,
///    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV = 2,
///} VkAccelerationStructureMemoryRequirementsTypeNV;
///```
///# Description
/// - [`OBJECT`] requests the memory requirement for the [`AccelerationStructureNV`] backing store.
/// - [`BUILD_SCRATCH`] requests the memory requirement for scratch space during the initial build.
/// - [`UPDATE_SCRATCH`] requests the memory requirement for scratch space during an update.
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl const Default for AccelerationStructureMemoryRequirementsTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    ///[`OBJECT`]
    ///requests the memory requirement for the [`AccelerationStructureNV`]
    ///backing store.
    pub const OBJECT: Self = Self(0);
    ///[`BUILD_SCRATCH`]
    ///requests the memory requirement for scratch space during the initial
    ///build.
    pub const BUILD_SCRATCH: Self = Self(1);
    ///[`UPDATE_SCRATCH`]
    ///requests the memory requirement for scratch space during an update.
    pub const UPDATE_SCRATCH: Self = Self(2);
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
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(AccelerationStructureMemoryRequirementsTypeNV))
            .field(match *self {
                Self::OBJECT => &"OBJECT",
                Self::BUILD_SCRATCH => &"BUILD_SCRATCH",
                Self::UPDATE_SCRATCH => &"UPDATE_SCRATCH",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(AccelerationStructureMemoryRequirementsTypeNV),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::OBJECT => &"OBJECT",
            Self::BUILD_SCRATCH => &"BUILD_SCRATCH",
            Self::UPDATE_SCRATCH => &"UPDATE_SCRATCH",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(AccelerationStructureMemoryRequirementsTypeNV),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkRayTracingShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) - Structure specifying shaders in a shader group
///# C Specifications
///The [`RayTracingShaderGroupCreateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkRayTracingShaderGroupCreateInfoNV {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkRayTracingShaderGroupTypeKHR    type;
///    uint32_t                          generalShader;
///    uint32_t                          closestHitShader;
///    uint32_t                          anyHitShader;
///    uint32_t                          intersectionShader;
///} VkRayTracingShaderGroupCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is the type of hit group specified in this structure.
/// - [`general_shader`] is the index of the ray generation, miss, or callable shader from
///   [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`, and [`SHADER_UNUSED_NV`] otherwise.
/// - [`closest_hit_shader`] is the optional index of the closest hit shader from
///   [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
/// - [`any_hit_shader`] is the optional index of the any-hit shader from
///   [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
/// - [`intersection_shader`] is the index of the intersection shader from
///   [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
///# Description
///## Valid Usage
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`general_shader`] **must**
///   be a valid index into [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of
///   `VK_SHADER_STAGE_RAYGEN_BIT_NV`, `VK_SHADER_STAGE_MISS_BIT_NV`, or
///   `VK_SHADER_STAGE_CALLABLE_BIT_NV`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`closest_hit_shader`],
///   [`any_hit_shader`], and [`intersection_shader`] **must**  be [`SHADER_UNUSED_NV`]
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV` then
///   [`intersection_shader`] **must**  be a valid index into
///   [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of
///   `VK_SHADER_STAGE_INTERSECTION_BIT_NV`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` then
///   [`intersection_shader`] **must**  be [`SHADER_UNUSED_NV`]
/// - [`closest_hit_shader`] **must**  be either [`SHADER_UNUSED_NV`] or a valid index into
///   [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of
///   `VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`
/// - [`any_hit_shader`] **must**  be either [`SHADER_UNUSED_NV`] or a valid index into
///   [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of
///   `VK_SHADER_STAGE_ANY_HIT_BIT_NV`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`RayTracingShaderGroupTypeKHR`] value
///# Related
/// - [`nv_ray_tracing`]
/// - [`RayTracingPipelineCreateInfoNV`]
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
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV<'lt> {
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
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    pub general_shader: u32,
    ///[`closest_hit_shader`] is the optional index of the closest hit shader
    ///from [`RayTracingPipelineCreateInfoNV`]::`pStages` in the group
    ///if the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    pub closest_hit_shader: u32,
    ///[`any_hit_shader`] is the optional index of the any-hit shader from
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    pub any_hit_shader: u32,
    ///[`intersection_shader`] is the index of the intersection shader from
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    pub intersection_shader: u32,
}
impl<'lt> Default for RayTracingShaderGroupCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            type_: Default::default(),
            general_shader: 0,
            closest_hit_shader: 0,
            any_hit_shader: 0,
            intersection_shader: 0,
        }
    }
}
impl<'lt> RayTracingShaderGroupCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        mut self,
        value: crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    ) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::general_shader`]
    pub fn set_general_shader(mut self, value: u32) -> Self {
        self.general_shader = value;
        self
    }
    ///Sets the value of [`Self::closest_hit_shader`]
    pub fn set_closest_hit_shader(mut self, value: u32) -> Self {
        self.closest_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::any_hit_shader`]
    pub fn set_any_hit_shader(mut self, value: u32) -> Self {
        self.any_hit_shader = value;
        self
    }
    ///Sets the value of [`Self::intersection_shader`]
    pub fn set_intersection_shader(mut self, value: u32) -> Self {
        self.intersection_shader = value;
        self
    }
}
///[VkRayTracingPipelineCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoNV.html) - Structure specifying parameters of a newly created ray tracing pipeline
///# C Specifications
///The [`RayTracingPipelineCreateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkRayTracingPipelineCreateInfoNV {
///    VkStructureType                               sType;
///    const void*                                   pNext;
///    VkPipelineCreateFlags                         flags;
///    uint32_t                                      stageCount;
///    const VkPipelineShaderStageCreateInfo*        pStages;
///    uint32_t                                      groupCount;
///    const VkRayTracingShaderGroupCreateInfoNV*    pGroups;
///    uint32_t                                      maxRecursionDepth;
///    VkPipelineLayout                              layout;
///    VkPipeline                                    basePipelineHandle;
///    int32_t                                       basePipelineIndex;
///} VkRayTracingPipelineCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying how the pipeline will be
///   generated.
/// - [`stage_count`] is the number of entries in the [`stages`] array.
/// - [`stages`] is a pointer to an array of [`PipelineShaderStageCreateInfo`] structures specifying
///   the set of the shader stages to be included in the ray tracing pipeline.
/// - [`group_count`] is the number of entries in the [`groups`] array.
/// - [`groups`] is a pointer to an array of [`RayTracingShaderGroupCreateInfoNV`] structures
///   describing the set of the shader stages to be included in each shader group in the ray tracing
///   pipeline.
/// - [`max_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth)
///   of shaders executed by this pipeline.
/// - [`layout`] is the description of binding locations used by both the pipeline and descriptor
///   sets used with the pipeline.
/// - [`base_pipeline_handle`] is a pipeline to derive from.
/// - [`base_pipeline_index`] is an index into the `pCreateInfos` parameter to use as a pipeline to
///   derive from.
///# Description
///The parameters [`base_pipeline_handle`] and [`base_pipeline_index`] are
///described in more detail in [Pipeline
///Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).
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
/// - The `stage` member of at least one element of [`stages`] **must**  be
///   `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
/// - [`max_recursion_depth`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV`]::[`max_recursion_depth`]
/// - [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`
/// - [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`
/// - [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
/// - [`flags`] **must**  not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
/// - [`flags`] **must**  not include `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`
/// - [`flags`] **must**  not include both `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV` and
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` at the same time
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`PipelineCreationFeedbackCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`PipelineCreateFlagBits`] values
/// - [`stages`] **must**  be a valid pointer to an array of [`stage_count`] valid
///   [`PipelineShaderStageCreateInfo`] structures
/// - [`groups`] **must**  be a valid pointer to an array of [`group_count`] valid
///   [`RayTracingShaderGroupCreateInfoNV`] structures
/// - [`layout`] **must**  be a valid [`PipelineLayout`] handle
/// - [`stage_count`] **must**  be greater than `0`
/// - [`group_count`] **must**  be greater than `0`
/// - Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored
///   parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`nv_ray_tracing`]
/// - [`Pipeline`]
/// - [`PipelineCreateFlags`]
/// - [`PipelineLayout`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`RayTracingShaderGroupCreateInfoNV`]
/// - [`StructureType`]
/// - [`create_ray_tracing_pipelines_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV<'lt> {
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
    ///[`stages`] is a pointer to an array of
    ///[`PipelineShaderStageCreateInfo`] structures specifying the set of
    ///the shader stages to be included in the ray tracing pipeline.
    pub stages: *const PipelineShaderStageCreateInfo<'lt>,
    ///[`group_count`] is the number of entries in the [`groups`] array.
    pub group_count: u32,
    ///[`groups`] is a pointer to an array of
    ///[`RayTracingShaderGroupCreateInfoNV`] structures describing the set
    ///of the shader stages to be included in each shader group in the ray
    ///tracing pipeline.
    pub groups: *const RayTracingShaderGroupCreateInfoNV<'lt>,
    ///[`max_recursion_depth`] is the [maximum
    ///recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
    pub max_recursion_depth: u32,
    ///[`layout`] is the description of binding locations used by both the
    ///pipeline and descriptor sets used with the pipeline.
    pub layout: PipelineLayout,
    ///[`base_pipeline_handle`] is a pipeline to derive from.
    pub base_pipeline_handle: Pipeline,
    ///[`base_pipeline_index`] is an index into the `pCreateInfos`
    ///parameter to use as a pipeline to derive from.
    pub base_pipeline_index: i32,
}
impl<'lt> Default for RayTracingPipelineCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            stage_count: 0,
            stages: std::ptr::null(),
            group_count: 0,
            groups: std::ptr::null(),
            max_recursion_depth: 0,
            layout: Default::default(),
            base_pipeline_handle: Default::default(),
            base_pipeline_index: 0,
        }
    }
}
impl<'lt> RayTracingPipelineCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stages`]
    pub fn stages_raw(&self) -> *const PipelineShaderStageCreateInfo<'lt> {
        self.stages
    }
    ///Gets the raw value of [`Self::groups`]
    pub fn groups_raw(&self) -> *const RayTracingShaderGroupCreateInfoNV<'lt> {
        self.groups
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stages`]
    pub fn set_stages_raw(mut self, value: *const PipelineShaderStageCreateInfo<'lt>) -> Self {
        self.stages = value;
        self
    }
    ///Sets the raw value of [`Self::groups`]
    pub fn set_groups_raw(mut self, value: *const RayTracingShaderGroupCreateInfoNV<'lt>) -> Self {
        self.groups = value;
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
    pub unsafe fn groups(&self) -> &[RayTracingShaderGroupCreateInfoNV<'lt>] {
        std::slice::from_raw_parts(self.groups, self.group_count as usize)
    }
    ///Gets the value of [`Self::max_recursion_depth`]
    pub fn max_recursion_depth(&self) -> u32 {
        self.max_recursion_depth
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
    ///Gets a mutable reference to the value of [`Self::max_recursion_depth`]
    pub fn max_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_recursion_depth
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
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::vulkan1_0::PipelineCreateFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::stage_count`]
    pub fn set_stage_count(mut self, value: u32) -> Self {
        self.stage_count = value;
        self
    }
    ///Sets the value of [`Self::stages`]
    pub fn set_stages(mut self, value: &'lt [crate::vulkan1_0::PipelineShaderStageCreateInfo<'lt>]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.stages = value.as_ptr();
        self.stage_count = len_;
        self
    }
    ///Sets the value of [`Self::group_count`]
    pub fn set_group_count(mut self, value: u32) -> Self {
        self.group_count = value;
        self
    }
    ///Sets the value of [`Self::groups`]
    pub fn set_groups(
        mut self,
        value: &'lt [crate::extensions::nv_ray_tracing::RayTracingShaderGroupCreateInfoNV<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.groups = value.as_ptr();
        self.group_count = len_;
        self
    }
    ///Sets the value of [`Self::max_recursion_depth`]
    pub fn set_max_recursion_depth(mut self, value: u32) -> Self {
        self.max_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::layout`]
    pub fn set_layout(mut self, value: crate::vulkan1_0::PipelineLayout) -> Self {
        self.layout = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_handle`]
    pub fn set_base_pipeline_handle(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.base_pipeline_handle = value;
        self
    }
    ///Sets the value of [`Self::base_pipeline_index`]
    pub fn set_base_pipeline_index(mut self, value: i32) -> Self {
        self.base_pipeline_index = value;
        self
    }
}
unsafe impl<'lt> crate::Chain<'lt, PipelineCreationFeedbackCreateInfo<'lt>> for RayTracingPipelineCreateInfoNV<'lt> {}
///[VkGeometryTrianglesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTrianglesNV.html) - Structure specifying a triangle geometry in a bottom-level acceleration structure
///# C Specifications
///The [`GeometryTrianglesNV`] structure specifies triangle geometry in a
///bottom-level acceleration structure and is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkGeometryTrianglesNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBuffer           vertexData;
///    VkDeviceSize       vertexOffset;
///    uint32_t           vertexCount;
///    VkDeviceSize       vertexStride;
///    VkFormat           vertexFormat;
///    VkBuffer           indexData;
///    VkDeviceSize       indexOffset;
///    uint32_t           indexCount;
///    VkIndexType        indexType;
///    VkBuffer           transformData;
///    VkDeviceSize       transformOffset;
///} VkGeometryTrianglesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_data`] is the buffer containing vertex data for this geometry.
/// - [`vertex_offset`] is the offset in bytes within [`vertex_data`] containing vertex data for
///   this geometry.
/// - [`vertex_count`] is the number of valid vertices.
/// - [`vertex_stride`] is the stride in bytes between each vertex.
/// - [`vertex_format`] is a [`Format`] describing the format of each vertex element.
/// - [`index_data`] is the buffer containing index data for this geometry.
/// - [`index_offset`] is the offset in bytes within [`index_data`] containing index data for this
///   geometry.
/// - [`index_count`] is the number of indices to include in this geometry.
/// - [`index_type`] is a [`IndexType`] describing the format of each index.
/// - [`transform_data`] is an optional buffer containing an [`TransformMatrixNV`] structure
///   defining a transformation to be applied to this geometry.
/// - [`transform_offset`] is the offset in bytes in [`transform_data`] of the transform information
///   described above.
///# Description
///If [`index_type`] is `VK_INDEX_TYPE_NONE_NV`, then this structure
///describes a set of triangles determined by [`vertex_count`].
///Otherwise, this structure describes a set of indexed triangles determined by
///[`index_count`].
///## Valid Usage
/// - [`vertex_offset`] **must**  be less than the size of [`vertex_data`]
/// - [`vertex_offset`] **must**  be a multiple of the component size of [`vertex_format`]
/// - [`vertex_format`] **must**  be one of `VK_FORMAT_R32G32B32_SFLOAT`, `VK_FORMAT_R32G32_SFLOAT`,
///   `VK_FORMAT_R16G16B16_SFLOAT`, `VK_FORMAT_R16G16_SFLOAT`, `VK_FORMAT_R16G16_SNORM`, or
///   `VK_FORMAT_R16G16B16_SNORM`
/// - [`vertex_stride`] **must**  be less than or equal to 2<sup>32</sup>-1
/// - [`index_offset`] **must**  be less than the size of [`index_data`]
/// - [`index_offset`] **must**  be a multiple of the element size of [`index_type`]
/// - [`index_type`] **must**  be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_data`] **must**  be [`crate::Handle::null`] if [`index_type`] is
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_data`] **must**  be a valid [`Buffer`] handle if [`index_type`] is not
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_count`] **must**  be `0` if [`index_type`] is `VK_INDEX_TYPE_NONE_NV`
/// - [`transform_offset`] **must**  be less than the size of [`transform_data`]
/// - [`transform_offset`] **must**  be a multiple of `16`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
/// - [`p_next`] **must**  be `NULL`
/// - If [`vertex_data`] is not [`crate::Handle::null`], [`vertex_data`] **must**  be a valid
///   [`Buffer`] handle
/// - [`vertex_format`] **must**  be a valid [`Format`] value
/// - If [`index_data`] is not [`crate::Handle::null`], [`index_data`] **must**  be a valid
///   [`Buffer`] handle
/// - [`index_type`] **must**  be a valid [`IndexType`] value
/// - If [`transform_data`] is not [`crate::Handle::null`], [`transform_data`] **must**  be a valid
///   [`Buffer`] handle
/// - Each of [`index_data`], [`transform_data`], and [`vertex_data`] that are valid handles of
///   non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`nv_ray_tracing`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`Format`]
/// - [`GeometryDataNV`]
/// - [`IndexType`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeometryTrianglesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vertex_data`] is the buffer containing vertex data for this geometry.
    pub vertex_data: Buffer,
    ///[`vertex_offset`] is the offset in bytes within [`vertex_data`]
    ///containing vertex data for this geometry.
    pub vertex_offset: DeviceSize,
    ///[`vertex_count`] is the number of valid vertices.
    pub vertex_count: u32,
    ///[`vertex_stride`] is the stride in bytes between each vertex.
    pub vertex_stride: DeviceSize,
    ///[`vertex_format`] is a [`Format`] describing the format of each
    ///vertex element.
    pub vertex_format: Format,
    ///[`index_data`] is the buffer containing index data for this geometry.
    pub index_data: Buffer,
    ///[`index_offset`] is the offset in bytes within [`index_data`]
    ///containing index data for this geometry.
    pub index_offset: DeviceSize,
    ///[`index_count`] is the number of indices to include in this geometry.
    pub index_count: u32,
    ///[`index_type`] is a [`IndexType`] describing the format of each
    ///index.
    pub index_type: IndexType,
    ///[`transform_data`] is an optional buffer containing an
    ///[`TransformMatrixNV`] structure defining a transformation to be
    ///applied to this geometry.
    pub transform_data: Buffer,
    ///[`transform_offset`] is the offset in bytes in [`transform_data`] of
    ///the transform information described above.
    pub transform_offset: DeviceSize,
}
impl<'lt> Default for GeometryTrianglesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GEOMETRY_TRIANGLES_NV,
            p_next: std::ptr::null(),
            vertex_data: Default::default(),
            vertex_offset: Default::default(),
            vertex_count: 0,
            vertex_stride: Default::default(),
            vertex_format: Default::default(),
            index_data: Default::default(),
            index_offset: Default::default(),
            index_count: 0,
            index_type: Default::default(),
            transform_data: Default::default(),
            transform_offset: Default::default(),
        }
    }
}
impl<'lt> GeometryTrianglesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::vertex_data`]
    pub fn vertex_data(&self) -> Buffer {
        self.vertex_data
    }
    ///Gets the value of [`Self::vertex_offset`]
    pub fn vertex_offset(&self) -> DeviceSize {
        self.vertex_offset
    }
    ///Gets the value of [`Self::vertex_count`]
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    ///Gets the value of [`Self::vertex_stride`]
    pub fn vertex_stride(&self) -> DeviceSize {
        self.vertex_stride
    }
    ///Gets the value of [`Self::vertex_format`]
    pub fn vertex_format(&self) -> Format {
        self.vertex_format
    }
    ///Gets the value of [`Self::index_data`]
    pub fn index_data(&self) -> Buffer {
        self.index_data
    }
    ///Gets the value of [`Self::index_offset`]
    pub fn index_offset(&self) -> DeviceSize {
        self.index_offset
    }
    ///Gets the value of [`Self::index_count`]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    ///Gets the value of [`Self::index_type`]
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Gets the value of [`Self::transform_data`]
    pub fn transform_data(&self) -> Buffer {
        self.transform_data
    }
    ///Gets the value of [`Self::transform_offset`]
    pub fn transform_offset(&self) -> DeviceSize {
        self.transform_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vertex_data`]
    pub fn vertex_data_mut(&mut self) -> &mut Buffer {
        &mut self.vertex_data
    }
    ///Gets a mutable reference to the value of [`Self::vertex_offset`]
    pub fn vertex_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_offset
    }
    ///Gets a mutable reference to the value of [`Self::vertex_count`]
    pub fn vertex_count_mut(&mut self) -> &mut u32 {
        &mut self.vertex_count
    }
    ///Gets a mutable reference to the value of [`Self::vertex_stride`]
    pub fn vertex_stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_stride
    }
    ///Gets a mutable reference to the value of [`Self::vertex_format`]
    pub fn vertex_format_mut(&mut self) -> &mut Format {
        &mut self.vertex_format
    }
    ///Gets a mutable reference to the value of [`Self::index_data`]
    pub fn index_data_mut(&mut self) -> &mut Buffer {
        &mut self.index_data
    }
    ///Gets a mutable reference to the value of [`Self::index_offset`]
    pub fn index_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.index_offset
    }
    ///Gets a mutable reference to the value of [`Self::index_count`]
    pub fn index_count_mut(&mut self) -> &mut u32 {
        &mut self.index_count
    }
    ///Gets a mutable reference to the value of [`Self::index_type`]
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Gets a mutable reference to the value of [`Self::transform_data`]
    pub fn transform_data_mut(&mut self) -> &mut Buffer {
        &mut self.transform_data
    }
    ///Gets a mutable reference to the value of [`Self::transform_offset`]
    pub fn transform_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.transform_offset
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::vertex_data`]
    pub fn set_vertex_data(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.vertex_data = value;
        self
    }
    ///Sets the value of [`Self::vertex_offset`]
    pub fn set_vertex_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.vertex_offset = value;
        self
    }
    ///Sets the value of [`Self::vertex_count`]
    pub fn set_vertex_count(mut self, value: u32) -> Self {
        self.vertex_count = value;
        self
    }
    ///Sets the value of [`Self::vertex_stride`]
    pub fn set_vertex_stride(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.vertex_stride = value;
        self
    }
    ///Sets the value of [`Self::vertex_format`]
    pub fn set_vertex_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.vertex_format = value;
        self
    }
    ///Sets the value of [`Self::index_data`]
    pub fn set_index_data(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.index_data = value;
        self
    }
    ///Sets the value of [`Self::index_offset`]
    pub fn set_index_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.index_offset = value;
        self
    }
    ///Sets the value of [`Self::index_count`]
    pub fn set_index_count(mut self, value: u32) -> Self {
        self.index_count = value;
        self
    }
    ///Sets the value of [`Self::index_type`]
    pub fn set_index_type(mut self, value: crate::vulkan1_0::IndexType) -> Self {
        self.index_type = value;
        self
    }
    ///Sets the value of [`Self::transform_data`]
    pub fn set_transform_data(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.transform_data = value;
        self
    }
    ///Sets the value of [`Self::transform_offset`]
    pub fn set_transform_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.transform_offset = value;
        self
    }
}
///[VkGeometryAABBNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryAABBNV.html) - Structure specifying axis-aligned bounding box geometry in a bottom-level acceleration structure
///# C Specifications
///The [`GeometryAabbNV`] structure specifies axis-aligned bounding box
///geometry in a bottom-level acceleration structure, and is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkGeometryAABBNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBuffer           aabbData;
///    uint32_t           numAABBs;
///    uint32_t           stride;
///    VkDeviceSize       offset;
///} VkGeometryAABBNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`aabb_data`] is the buffer containing axis-aligned bounding box data.
/// - [`num_aab_bs`] is the number of AABBs in this geometry.
/// - [`stride`] is the stride in bytes between AABBs in [`aabb_data`].
/// - [`offset`] is the offset in bytes of the first AABB in [`aabb_data`].
///# Description
///The AABB data in memory is six 32-bit floats consisting of the minimum x, y,
///and z values followed by the maximum x, y, and z values.
///## Valid Usage
/// - [`offset`] **must**  be less than the size of [`aabb_data`]
/// - [`offset`] **must**  be a multiple of `8`
/// - [`stride`] **must**  be a multiple of `8`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
/// - [`p_next`] **must**  be `NULL`
/// - If [`aabb_data`] is not [`crate::Handle::null`], [`aabb_data`] **must**  be a valid [`Buffer`]
///   handle
///# Related
/// - [`nv_ray_tracing`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`GeometryDataNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeometryAabbNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`aabb_data`] is the buffer containing axis-aligned bounding box data.
    pub aabb_data: Buffer,
    ///[`num_aab_bs`] is the number of AABBs in this geometry.
    pub num_aab_bs: u32,
    ///[`stride`] is the stride in bytes between AABBs in [`aabb_data`].
    pub stride: u32,
    ///[`offset`] is the offset in bytes of the first AABB in [`aabb_data`].
    pub offset: DeviceSize,
}
impl<'lt> Default for GeometryAabbNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GEOMETRY_AABB_NV,
            p_next: std::ptr::null(),
            aabb_data: Default::default(),
            num_aab_bs: 0,
            stride: 0,
            offset: Default::default(),
        }
    }
}
impl<'lt> GeometryAabbNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::aabb_data`]
    pub fn aabb_data(&self) -> Buffer {
        self.aabb_data
    }
    ///Gets the value of [`Self::num_aab_bs`]
    pub fn num_aab_bs(&self) -> u32 {
        self.num_aab_bs
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::aabb_data`]
    pub fn aabb_data_mut(&mut self) -> &mut Buffer {
        &mut self.aabb_data
    }
    ///Gets a mutable reference to the value of [`Self::num_aab_bs`]
    pub fn num_aab_bs_mut(&mut self) -> &mut u32 {
        &mut self.num_aab_bs
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::aabb_data`]
    pub fn set_aabb_data(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.aabb_data = value;
        self
    }
    ///Sets the value of [`Self::num_aab_bs`]
    pub fn set_num_aab_bs(mut self, value: u32) -> Self {
        self.num_aab_bs = value;
        self
    }
    ///Sets the value of [`Self::stride`]
    pub fn set_stride(mut self, value: u32) -> Self {
        self.stride = value;
        self
    }
    ///Sets the value of [`Self::offset`]
    pub fn set_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.offset = value;
        self
    }
}
///[VkGeometryDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html) - Structure specifying geometry in a bottom-level acceleration structure
///# C Specifications
///The [`GeometryDataNV`] structure specifes geometry in a bottom-level
///acceleration structure and is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkGeometryDataNV {
///    VkGeometryTrianglesNV    triangles;
///    VkGeometryAABBNV         aabbs;
///} VkGeometryDataNV;
///```
///# Members
/// - [`triangles`] contains triangle data if [`GeometryNV::geometry_type`] is
///   `VK_GEOMETRY_TYPE_TRIANGLES_NV`.
/// - [`aabbs`] contains axis-aligned bounding box data if [`GeometryNV::geometry_type`] is
///   `VK_GEOMETRY_TYPE_AABBS_NV`.
///# Description
///## Valid Usage (Implicit)
/// - [`triangles`] **must**  be a valid [`GeometryTrianglesNV`] structure
/// - [`aabbs`] **must**  be a valid [`GeometryAabbNV`] structure
///# Related
/// - [`nv_ray_tracing`]
/// - [`GeometryAabbNV`]
/// - [`GeometryNV`]
/// - [`GeometryTrianglesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryDataNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeometryDataNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`triangles`] contains triangle data if
    ///[`GeometryNV`]::`geometryType` is
    ///`VK_GEOMETRY_TYPE_TRIANGLES_NV`.
    pub triangles: GeometryTrianglesNV<'lt>,
    ///[`aabbs`] contains axis-aligned bounding box data if
    ///[`GeometryNV`]::`geometryType` is
    ///`VK_GEOMETRY_TYPE_AABBS_NV`.
    pub aabbs: GeometryAabbNV<'lt>,
}
impl<'lt> Default for GeometryDataNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            triangles: Default::default(),
            aabbs: Default::default(),
        }
    }
}
impl<'lt> GeometryDataNV<'lt> {
    ///Gets the value of [`Self::triangles`]
    pub fn triangles(&self) -> &GeometryTrianglesNV<'lt> {
        &self.triangles
    }
    ///Gets the value of [`Self::aabbs`]
    pub fn aabbs(&self) -> &GeometryAabbNV<'lt> {
        &self.aabbs
    }
    ///Gets a mutable reference to the value of [`Self::triangles`]
    pub fn triangles_mut(&mut self) -> &mut GeometryTrianglesNV<'lt> {
        &mut self.triangles
    }
    ///Gets a mutable reference to the value of [`Self::aabbs`]
    pub fn aabbs_mut(&mut self) -> &mut GeometryAabbNV<'lt> {
        &mut self.aabbs
    }
    ///Sets the value of [`Self::triangles`]
    pub fn set_triangles(mut self, value: crate::extensions::nv_ray_tracing::GeometryTrianglesNV<'lt>) -> Self {
        self.triangles = value;
        self
    }
    ///Sets the value of [`Self::aabbs`]
    pub fn set_aabbs(mut self, value: crate::extensions::nv_ray_tracing::GeometryAabbNV<'lt>) -> Self {
        self.aabbs = value;
        self
    }
}
///[VkGeometryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryNV.html) - Structure specifying a geometry in a bottom-level acceleration structure
///# C Specifications
///The [`GeometryNV`] structure describes geometry in a bottom-level
///acceleration structure and is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkGeometryNV {
///    VkStructureType       sType;
///    const void*           pNext;
///    VkGeometryTypeKHR     geometryType;
///    VkGeometryDataNV      geometry;
///    VkGeometryFlagsKHR    flags;
///} VkGeometryNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`geometry_type`] specifies the [`GeometryTypeKHR`] which this geometry refers to.
/// - [`geometry`] contains the geometry data as described in [`GeometryDataNV`].
/// - [`flags`] has [`GeometryFlagBitsKHR`] describing options for this geometry.
///# Description
///## Valid Usage
/// - [`geometry_type`] **must**  be `VK_GEOMETRY_TYPE_TRIANGLES_NV` or `VK_GEOMETRY_TYPE_AABBS_NV`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`geometry_type`] **must**  be a valid [`GeometryTypeKHR`] value
/// - [`geometry`] **must**  be a valid [`GeometryDataNV`] structure
/// - [`flags`] **must**  be a valid combination of [`GeometryFlagBitsKHR`] values
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureInfoNV`]
/// - [`GeometryDataNV`]
/// - [`GeometryFlagsKHR`]
/// - [`GeometryTypeKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeometryNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`geometry_type`] specifies the [`GeometryTypeKHR`] which this
    ///geometry refers to.
    pub geometry_type: GeometryTypeKHR,
    ///[`geometry`] contains the geometry data as described in
    ///[`GeometryDataNV`].
    pub geometry: GeometryDataNV<'lt>,
    ///[`flags`] has [`GeometryFlagBitsKHR`] describing options for this
    ///geometry.
    pub flags: GeometryFlagsKHR,
}
impl<'lt> Default for GeometryNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GEOMETRY_NV,
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> GeometryNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::geometry_type`]
    pub fn geometry_type(&self) -> GeometryTypeKHR {
        self.geometry_type
    }
    ///Gets the value of [`Self::geometry`]
    pub fn geometry(&self) -> &GeometryDataNV<'lt> {
        &self.geometry
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> GeometryFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::geometry_type`]
    pub fn geometry_type_mut(&mut self) -> &mut GeometryTypeKHR {
        &mut self.geometry_type
    }
    ///Gets a mutable reference to the value of [`Self::geometry`]
    pub fn geometry_mut(&mut self) -> &mut GeometryDataNV<'lt> {
        &mut self.geometry
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut GeometryFlagsKHR {
        &mut self.flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::geometry_type`]
    pub fn set_geometry_type(mut self, value: crate::extensions::khr_acceleration_structure::GeometryTypeKHR) -> Self {
        self.geometry_type = value;
        self
    }
    ///Sets the value of [`Self::geometry`]
    pub fn set_geometry(mut self, value: crate::extensions::nv_ray_tracing::GeometryDataNV<'lt>) -> Self {
        self.geometry = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR) -> Self {
        self.flags = value;
        self
    }
}
///[VkAccelerationStructureInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInfoNV.html) - Structure specifying the parameters of acceleration structure object
///# C Specifications
///The [`AccelerationStructureInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkAccelerationStructureInfoNV {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    VkAccelerationStructureTypeNV          type;
///    VkBuildAccelerationStructureFlagsNV    flags;
///    uint32_t                               instanceCount;
///    uint32_t                               geometryCount;
///    const VkGeometryNV*                    pGeometries;
///} VkAccelerationStructureInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is a [`AccelerationStructureTypeNV`] value specifying the type of acceleration
///   structure that will be created.
/// - [`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsNV`] specifying additional
///   parameters of the acceleration structure.
/// - [`instance_count`] specifies the number of instances that will be in the new acceleration
///   structure.
/// - [`geometry_count`] specifies the number of geometries that will be in the new acceleration
///   structure.
/// - [`geometries`] is a pointer to an array of [`geometry_count`][`GeometryNV`] structures
///   containing the scene data being passed into the acceleration structure.
///# Description
///[`AccelerationStructureInfoNV`] contains information that is used both
///for acceleration structure creation with
///[`create_acceleration_structure_nv`] and in combination with the actual
///geometric data to build the acceleration structure with
///[`cmd_build_acceleration_structure_nv`].
///## Valid Usage
/// - [`geometry_count`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
/// - [`instance_count`] **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_instance_count`]
/// - The total number of triangles in all geometries  **must**  be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_triangle_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV` then [`geometry_count`] **must**
///   be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then [`instance_count`]
///   **must**  be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then the `geometryType`
///   member of each geometry in [`geometries`] **must**  be the same
/// - [`type_`] **must**  not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV` bit set, then
///   it  **must**  not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV` bit set
/// - `scratch` **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
/// - If `instanceData` is not [`crate::Handle::null`], `instanceData` **must**  have been created
///   with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`AccelerationStructureTypeNV`] value
/// - [`flags`] **must**  be a valid combination of [`BuildAccelerationStructureFlagBitsNV`] values
/// - If [`geometry_count`] is not `0`, [`geometries`] **must**  be a valid pointer to an array of
///   [`geometry_count`] valid [`GeometryNV`] structures
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureCreateInfoNV`]
/// - [`AccelerationStructureTypeNV`]
/// - [`BuildAccelerationStructureFlagsNV`]
/// - [`GeometryNV`]
/// - [`StructureType`]
/// - [`cmd_build_acceleration_structure_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AccelerationStructureInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is a [`AccelerationStructureTypeNV`] value specifying the
    ///type of acceleration structure that will be created.
    pub type_: AccelerationStructureTypeKHR,
    ///[`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsNV`]
    ///specifying additional parameters of the acceleration structure.
    pub flags: BuildAccelerationStructureFlagsKHR,
    ///[`instance_count`] specifies the number of instances that will be in
    ///the new acceleration structure.
    pub instance_count: u32,
    ///[`geometry_count`] specifies the number of geometries that will be in
    ///the new acceleration structure.
    pub geometry_count: u32,
    ///[`geometries`] is a pointer to an array of [`geometry_count`][`GeometryNV`] structures
    /// containing the scene data being passed into the acceleration structure.
    pub geometries: *const GeometryNV<'lt>,
}
impl<'lt> Default for AccelerationStructureInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ACCELERATION_STRUCTURE_INFO_NV,
            p_next: std::ptr::null(),
            type_: Default::default(),
            flags: Default::default(),
            instance_count: 0,
            geometry_count: 0,
            geometries: std::ptr::null(),
        }
    }
}
impl<'lt> AccelerationStructureInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::geometries`]
    pub fn geometries_raw(&self) -> *const GeometryNV<'lt> {
        self.geometries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::geometries`]
    pub fn set_geometries_raw(mut self, value: *const GeometryNV<'lt>) -> Self {
        self.geometries = value;
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
    pub fn type_(&self) -> AccelerationStructureTypeKHR {
        self.type_
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> BuildAccelerationStructureFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::instance_count`]
    pub fn instance_count(&self) -> u32 {
        self.instance_count
    }
    ///Gets the value of [`Self::geometry_count`]
    pub fn geometry_count(&self) -> u32 {
        self.geometry_count
    }
    ///Gets the value of [`Self::geometries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn geometries(&self) -> &[GeometryNV<'lt>] {
        std::slice::from_raw_parts(self.geometries, self.geometry_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut AccelerationStructureTypeKHR {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut BuildAccelerationStructureFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::instance_count`]
    pub fn instance_count_mut(&mut self) -> &mut u32 {
        &mut self.instance_count
    }
    ///Gets a mutable reference to the value of [`Self::geometry_count`]
    pub fn geometry_count_mut(&mut self) -> &mut u32 {
        &mut self.geometry_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR,
    ) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::instance_count`]
    pub fn set_instance_count(mut self, value: u32) -> Self {
        self.instance_count = value;
        self
    }
    ///Sets the value of [`Self::geometry_count`]
    pub fn set_geometry_count(mut self, value: u32) -> Self {
        self.geometry_count = value;
        self
    }
    ///Sets the value of [`Self::geometries`]
    pub fn set_geometries(mut self, value: &'lt [crate::extensions::nv_ray_tracing::GeometryNV<'lt>]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.geometries = value.as_ptr();
        self.geometry_count = len_;
        self
    }
}
///[VkAccelerationStructureCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoNV.html) - Structure specifying the parameters of a newly created acceleration structure object
///# C Specifications
///The [`AccelerationStructureCreateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkAccelerationStructureCreateInfoNV {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkDeviceSize                     compactedSize;
///    VkAccelerationStructureInfoNV    info;
///} VkAccelerationStructureCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`compacted_size`] is the size from the result of
///   [`cmd_write_acceleration_structures_properties_nv`] if this acceleration structure is going to
///   be the target of a compacting copy.
/// - [`info`] is the [`AccelerationStructureInfoNV`] structure specifying further parameters of the
///   created acceleration structure.
///# Description
///## Valid Usage
/// - If [`compacted_size`] is not `0` then both `info.geometryCount` and `info.instanceCount`
///   **must**  be `0`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`info`] **must**  be a valid [`AccelerationStructureInfoNV`] structure
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureInfoNV`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`create_acceleration_structure_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`compacted_size`] is the size from the result of
    ///[`cmd_write_acceleration_structures_properties_nv`] if this acceleration
    ///structure is going to be the target of a compacting copy.
    pub compacted_size: DeviceSize,
    ///[`info`] is the [`AccelerationStructureInfoNV`] structure
    ///specifying further parameters of the created acceleration structure.
    pub info: AccelerationStructureInfoNV<'lt>,
}
impl<'lt> Default for AccelerationStructureCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            compacted_size: Default::default(),
            info: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::compacted_size`]
    pub fn compacted_size(&self) -> DeviceSize {
        self.compacted_size
    }
    ///Gets the value of [`Self::info`]
    pub fn info(&self) -> &AccelerationStructureInfoNV<'lt> {
        &self.info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::compacted_size`]
    pub fn compacted_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.compacted_size
    }
    ///Gets a mutable reference to the value of [`Self::info`]
    pub fn info_mut(&mut self) -> &mut AccelerationStructureInfoNV<'lt> {
        &mut self.info
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::compacted_size`]
    pub fn set_compacted_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.compacted_size = value;
        self
    }
    ///Sets the value of [`Self::info`]
    pub fn set_info(mut self, value: crate::extensions::nv_ray_tracing::AccelerationStructureInfoNV<'lt>) -> Self {
        self.info = value;
        self
    }
}
///[VkBindAccelerationStructureMemoryInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) - Structure specifying acceleration structure memory binding
///# C Specifications
///The [`BindAccelerationStructureMemoryInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkBindAccelerationStructureMemoryInfoNV {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkAccelerationStructureNV    accelerationStructure;
///    VkDeviceMemory               memory;
///    VkDeviceSize                 memoryOffset;
///    uint32_t                     deviceIndexCount;
///    const uint32_t*              pDeviceIndices;
///} VkBindAccelerationStructureMemoryInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure`] is the acceleration structure to be attached to memory.
/// - [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
/// - [`memory_offset`] is the start offset of the region of memory that is to be bound to the
///   acceleration structure. The number of bytes returned in the [`MemoryRequirements::size`]
///   member in [`memory`], starting from [`memory_offset`] bytes, will be bound to the specified
///   acceleration structure.
/// - [`device_index_count`] is the number of elements in [`device_indices`].
/// - [`device_indices`] is a pointer to an array of device indices.
///# Description
///## Valid Usage
/// - [`acceleration_structure`] **must**  not already be backed by a memory object
/// - [`memory_offset`] **must**  be less than the size of [`memory`]
/// - [`memory`] **must**  have been allocated using one of the memory types allowed in the
///   `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to
///   [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and
///   `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
/// - [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to
///   [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and
///   `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
/// - The `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and
///   `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV` **must**  be less
///   than or equal to the size of [`memory`] minus [`memory_offset`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an
///   array of [`device_index_count`]`uint32_t` values
/// - Both of [`acceleration_structure`], and [`memory`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`bind_acceleration_structure_memory_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`acceleration_structure`] is the acceleration structure to be attached
    ///to memory.
    pub acceleration_structure: AccelerationStructureNV,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    pub memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of memory that is
    ///to be bound to the acceleration structure.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified
    ///acceleration structure.
    pub memory_offset: DeviceSize,
    ///[`device_index_count`] is the number of elements in
    ///[`device_indices`].
    pub device_index_count: u32,
    ///[`device_indices`] is a pointer to an array of device indices.
    pub device_indices: *const u32,
}
impl<'lt> Default for BindAccelerationStructureMemoryInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            device_index_count: 0,
            device_indices: std::ptr::null(),
        }
    }
}
impl<'lt> BindAccelerationStructureMemoryInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_indices`]
    pub fn device_indices_raw(&self) -> *const u32 {
        self.device_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_indices`]
    pub fn set_device_indices_raw(mut self, value: *const u32) -> Self {
        self.device_indices = value;
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
    ///Gets the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure(&self) -> AccelerationStructureNV {
        self.acceleration_structure
    }
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::memory_offset`]
    pub fn memory_offset(&self) -> DeviceSize {
        self.memory_offset
    }
    ///Gets the value of [`Self::device_index_count`]
    pub fn device_index_count(&self) -> u32 {
        self.device_index_count
    }
    ///Gets the value of [`Self::device_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.device_indices, self.device_index_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureNV {
        &mut self.acceleration_structure
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::memory_offset`]
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Gets a mutable reference to the value of [`Self::device_index_count`]
    pub fn device_index_count_mut(&mut self) -> &mut u32 {
        &mut self.device_index_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::acceleration_structure`]
    pub fn set_acceleration_structure(
        mut self,
        value: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    ) -> Self {
        self.acceleration_structure = value;
        self
    }
    ///Sets the value of [`Self::memory`]
    pub fn set_memory(mut self, value: crate::vulkan1_0::DeviceMemory) -> Self {
        self.memory = value;
        self
    }
    ///Sets the value of [`Self::memory_offset`]
    pub fn set_memory_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.memory_offset = value;
        self
    }
    ///Sets the value of [`Self::device_index_count`]
    pub fn set_device_index_count(mut self, value: u32) -> Self {
        self.device_index_count = value;
        self
    }
    ///Sets the value of [`Self::device_indices`]
    pub fn set_device_indices(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_indices = value.as_ptr();
        self.device_index_count = len_;
        self
    }
}
///[VkWriteDescriptorSetAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureNV.html) - Structure specifying acceleration structure descriptor information
///# C Specifications
///The [`WriteDescriptorSetAccelerationStructureNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkWriteDescriptorSetAccelerationStructureNV {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    uint32_t                            accelerationStructureCount;
///    const VkAccelerationStructureNV*    pAccelerationStructures;
///} VkWriteDescriptorSetAccelerationStructureNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure_count`] is the number of elements in [`acceleration_structures`].
/// - [`acceleration_structures`] is a pointer to an array of [`AccelerationStructureNV`] structures
///   specifying the acceleration structures to update.
///# Description
///## Valid Usage
/// - [`acceleration_structure_count`] **must**  be equal to `descriptorCount` in the extended
///   structure
/// - Each acceleration structure in [`acceleration_structures`] **must**  have been created with
///   `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`
/// - If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor)
///   feature is not enabled, each member of [`acceleration_structures`] **must**  not be
///   [`crate::Handle::null`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
/// - [`acceleration_structures`] **must**  be a valid pointer to an array of
///   [`acceleration_structure_count`] valid or [`crate::Handle::null`][`AccelerationStructureNV`]
///   handles
/// - [`acceleration_structure_count`] **must**  be greater than `0`
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`acceleration_structure_count`] is the number of elements in
    ///[`acceleration_structures`].
    pub acceleration_structure_count: u32,
    ///[`acceleration_structures`] is a pointer to an array of
    ///[`AccelerationStructureNV`] structures specifying the acceleration
    ///structures to update.
    pub acceleration_structures: *const AccelerationStructureNV,
}
impl<'lt> Default for WriteDescriptorSetAccelerationStructureNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
            p_next: std::ptr::null(),
            acceleration_structure_count: 0,
            acceleration_structures: std::ptr::null(),
        }
    }
}
impl<'lt> WriteDescriptorSetAccelerationStructureNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::acceleration_structures`]
    pub fn acceleration_structures_raw(&self) -> *const AccelerationStructureNV {
        self.acceleration_structures
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structures`]
    pub fn set_acceleration_structures_raw(mut self, value: *const AccelerationStructureNV) -> Self {
        self.acceleration_structures = value;
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
    ///Gets the value of [`Self::acceleration_structure_count`]
    pub fn acceleration_structure_count(&self) -> u32 {
        self.acceleration_structure_count
    }
    ///Gets the value of [`Self::acceleration_structures`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn acceleration_structures(&self) -> &[AccelerationStructureNV] {
        std::slice::from_raw_parts(self.acceleration_structures, self.acceleration_structure_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_count`]
    pub fn acceleration_structure_count_mut(&mut self) -> &mut u32 {
        &mut self.acceleration_structure_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::acceleration_structure_count`]
    pub fn set_acceleration_structure_count(mut self, value: u32) -> Self {
        self.acceleration_structure_count = value;
        self
    }
    ///Sets the value of [`Self::acceleration_structures`]
    pub fn set_acceleration_structures(
        mut self,
        value: &'lt [crate::extensions::nv_ray_tracing::AccelerationStructureNV],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.acceleration_structures = value.as_ptr();
        self.acceleration_structure_count = len_;
        self
    }
}
///[VkAccelerationStructureMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html) - Structure specifying acceleration to query for memory requirements
///# C Specifications
///The [`AccelerationStructureMemoryRequirementsInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkAccelerationStructureMemoryRequirementsInfoNV {
///    VkStructureType                                    sType;
///    const void*                                        pNext;
///    VkAccelerationStructureMemoryRequirementsTypeNV    type;
///    VkAccelerationStructureNV                          accelerationStructure;
///} VkAccelerationStructureMemoryRequirementsInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] selects the type of memory requirement being queried.
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV` returns the memory requirements
///   for the object itself. `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
///   returns the memory requirements for the scratch memory when doing a build.
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV` returns the memory
///   requirements for the scratch memory when doing an update.
/// - [`acceleration_structure`] is the acceleration structure to be queried for memory
///   requirements.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`AccelerationStructureMemoryRequirementsTypeNV`] value
/// - [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsTypeNV`]
/// - [`AccelerationStructureNV`]
/// - [`StructureType`]
/// - [`get_acceleration_structure_memory_requirements_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] selects the type of memory requirement being queried.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
    ///returns the memory requirements for the object itself.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
    ///returns the memory requirements for the scratch memory when doing a
    ///build.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV`
    ///returns the memory requirements for the scratch memory when doing an
    ///update.
    pub type_: AccelerationStructureMemoryRequirementsTypeNV,
    ///[`acceleration_structure`] is the acceleration structure to be queried
    ///for memory requirements.
    pub acceleration_structure: AccelerationStructureNV,
}
impl<'lt> Default for AccelerationStructureMemoryRequirementsInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            type_: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureMemoryRequirementsInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> AccelerationStructureMemoryRequirementsTypeNV {
        self.type_
    }
    ///Gets the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure(&self) -> AccelerationStructureNV {
        self.acceleration_structure
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut AccelerationStructureMemoryRequirementsTypeNV {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureNV {
        &mut self.acceleration_structure
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::type_`]
    pub fn set_type_(
        mut self,
        value: crate::extensions::nv_ray_tracing::AccelerationStructureMemoryRequirementsTypeNV,
    ) -> Self {
        self.type_ = value;
        self
    }
    ///Sets the value of [`Self::acceleration_structure`]
    pub fn set_acceleration_structure(
        mut self,
        value: crate::extensions::nv_ray_tracing::AccelerationStructureNV,
    ) -> Self {
        self.acceleration_structure = value;
        self
    }
}
///[VkPhysicalDeviceRayTracingPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) - Properties of the physical device for ray tracing
///# C Specifications
///The [`PhysicalDeviceRayTracingPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_ray_tracing
///typedef struct VkPhysicalDeviceRayTracingPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           shaderGroupHandleSize;
///    uint32_t           maxRecursionDepth;
///    uint32_t           maxShaderGroupStride;
///    uint32_t           shaderGroupBaseAlignment;
///    uint64_t           maxGeometryCount;
///    uint64_t           maxInstanceCount;
///    uint64_t           maxTriangleCount;
///    uint32_t           maxDescriptorSetAccelerationStructures;
///} VkPhysicalDeviceRayTracingPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_group_handle_size`] is the size in bytes of the shader header.
/// - [`max_recursion_depth`] is the maximum number of levels of recursion allowed in a trace
///   command.
/// - [`max_shader_group_stride`] is the maximum stride in bytes allowed between shader groups in
///   the shader binding table.
/// - [`shader_group_base_alignment`] is the  **required**  alignment in bytes for the base of the
///   shader binding table.
/// - [`max_geometry_count`] is the maximum number of geometries in the bottom level acceleration
///   structure.
/// - [`max_instance_count`] is the maximum number of instances in the top level acceleration
///   structure.
/// - [`max_triangle_count`] is the maximum number of triangles in all geometries in the bottom
///   level acceleration structure.
/// - [`max_descriptor_set_acceleration_structures`] is the maximum number of acceleration structure
///   descriptors that are allowed in a descriptor set.
///# Description
///Due to the fact that the geometry, instance, and triangle counts are
///specified at acceleration structure creation as 32-bit values,
///[`max_geometry_count`], [`max_instance_count`], and [`max_triangle_count`] **must**  not exceed
/// 2<sup>32</sup>-1.If the [`PhysicalDeviceRayTracingPropertiesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure  **must**
/// match those specified with the same
///name in [`PhysicalDeviceAccelerationStructurePropertiesKHR`] and
///[`PhysicalDeviceRayTracingPipelinePropertiesKHR`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`
///# Related
/// - [`nv_ray_tracing`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_group_handle_size`] is the size in bytes of the shader header.
    pub shader_group_handle_size: u32,
    ///[`max_recursion_depth`] is the maximum
    ///number of levels of recursion allowed in a trace command.
    pub max_recursion_depth: u32,
    ///[`max_shader_group_stride`] is the maximum stride in bytes allowed
    ///between shader groups in the shader binding table.
    pub max_shader_group_stride: u32,
    ///[`shader_group_base_alignment`] is the  **required**  alignment in bytes for
    ///the base of the shader binding table.
    pub shader_group_base_alignment: u32,
    ///[`max_geometry_count`] is the maximum number of geometries in the bottom
    ///level acceleration structure.
    pub max_geometry_count: u64,
    ///[`max_instance_count`] is the maximum number of instances in the top
    ///level acceleration structure.
    pub max_instance_count: u64,
    ///[`max_triangle_count`] is the maximum number of triangles in all
    ///geometries in the bottom level acceleration structure.
    pub max_triangle_count: u64,
    ///[`max_descriptor_set_acceleration_structures`] is the maximum number of
    ///acceleration structure descriptors that are allowed in a descriptor set.
    pub max_descriptor_set_acceleration_structures: u32,
}
impl<'lt> Default for PhysicalDeviceRayTracingPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_group_handle_size: 0,
            max_recursion_depth: 0,
            max_shader_group_stride: 0,
            shader_group_base_alignment: 0,
            max_geometry_count: 0,
            max_instance_count: 0,
            max_triangle_count: 0,
            max_descriptor_set_acceleration_structures: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRayTracingPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::max_recursion_depth`]
    pub fn max_recursion_depth(&self) -> u32 {
        self.max_recursion_depth
    }
    ///Gets the value of [`Self::max_shader_group_stride`]
    pub fn max_shader_group_stride(&self) -> u32 {
        self.max_shader_group_stride
    }
    ///Gets the value of [`Self::shader_group_base_alignment`]
    pub fn shader_group_base_alignment(&self) -> u32 {
        self.shader_group_base_alignment
    }
    ///Gets the value of [`Self::max_geometry_count`]
    pub fn max_geometry_count(&self) -> u64 {
        self.max_geometry_count
    }
    ///Gets the value of [`Self::max_instance_count`]
    pub fn max_instance_count(&self) -> u64 {
        self.max_instance_count
    }
    ///Gets the value of [`Self::max_triangle_count`]
    pub fn max_triangle_count(&self) -> u64 {
        self.max_triangle_count
    }
    ///Gets the value of [`Self::max_descriptor_set_acceleration_structures`]
    pub fn max_descriptor_set_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_acceleration_structures
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
    ///Gets a mutable reference to the value of [`Self::max_recursion_depth`]
    pub fn max_recursion_depth_mut(&mut self) -> &mut u32 {
        &mut self.max_recursion_depth
    }
    ///Gets a mutable reference to the value of [`Self::max_shader_group_stride`]
    pub fn max_shader_group_stride_mut(&mut self) -> &mut u32 {
        &mut self.max_shader_group_stride
    }
    ///Gets a mutable reference to the value of [`Self::shader_group_base_alignment`]
    pub fn shader_group_base_alignment_mut(&mut self) -> &mut u32 {
        &mut self.shader_group_base_alignment
    }
    ///Gets a mutable reference to the value of [`Self::max_geometry_count`]
    pub fn max_geometry_count_mut(&mut self) -> &mut u64 {
        &mut self.max_geometry_count
    }
    ///Gets a mutable reference to the value of [`Self::max_instance_count`]
    pub fn max_instance_count_mut(&mut self) -> &mut u64 {
        &mut self.max_instance_count
    }
    ///Gets a mutable reference to the value of [`Self::max_triangle_count`]
    pub fn max_triangle_count_mut(&mut self) -> &mut u64 {
        &mut self.max_triangle_count
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_acceleration_structures`]
    pub fn max_descriptor_set_acceleration_structures_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_acceleration_structures
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_group_handle_size`]
    pub fn set_shader_group_handle_size(mut self, value: u32) -> Self {
        self.shader_group_handle_size = value;
        self
    }
    ///Sets the value of [`Self::max_recursion_depth`]
    pub fn set_max_recursion_depth(mut self, value: u32) -> Self {
        self.max_recursion_depth = value;
        self
    }
    ///Sets the value of [`Self::max_shader_group_stride`]
    pub fn set_max_shader_group_stride(mut self, value: u32) -> Self {
        self.max_shader_group_stride = value;
        self
    }
    ///Sets the value of [`Self::shader_group_base_alignment`]
    pub fn set_shader_group_base_alignment(mut self, value: u32) -> Self {
        self.shader_group_base_alignment = value;
        self
    }
    ///Sets the value of [`Self::max_geometry_count`]
    pub fn set_max_geometry_count(mut self, value: u64) -> Self {
        self.max_geometry_count = value;
        self
    }
    ///Sets the value of [`Self::max_instance_count`]
    pub fn set_max_instance_count(mut self, value: u64) -> Self {
        self.max_instance_count = value;
        self
    }
    ///Sets the value of [`Self::max_triangle_count`]
    pub fn set_max_triangle_count(mut self, value: u64) -> Self {
        self.max_triangle_count = value;
        self
    }
    ///Sets the value of [`Self::max_descriptor_set_acceleration_structures`]
    pub fn set_max_descriptor_set_acceleration_structures(mut self, value: u32) -> Self {
        self.max_descriptor_set_acceleration_structures = value;
        self
    }
}
impl Device {
    ///[vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html) - Deferred compilation of shaders
    ///# C Specifications
    ///To compile a deferred shader in a pipeline call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkCompileDeferredNV(
    ///    VkDevice                                    device,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    shader);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device containing the ray tracing pipeline.
    /// - [`pipeline`] is the ray tracing pipeline object containing the shaders.
    /// - [`shader`] is the index of the shader to compile.
    ///# Description
    ///## Valid Usage
    /// - [`pipeline`] **must**  be a ray tracing pipeline
    /// - [`pipeline`] **must**  have been created with `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
    /// - [`shader`] **must**  not have been called as a deferred compile before
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
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
    #[doc(alias = "vkCompileDeferredNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn compile_deferred_nv(
        self: &Unique<Device>,
        pipeline: Pipeline,
        shader: Option<u32>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.compile_deferred_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.compile_deferred_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), pipeline, shader.unwrap_or_default() as _);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkCreateAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html) - Create a new acceleration structure object
    ///# C Specifications
    ///To create acceleration structures, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkCreateAccelerationStructureNV(
    ///    VkDevice                                    device,
    ///    const VkAccelerationStructureCreateInfoNV*  pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkAccelerationStructureNV*                  pAccelerationStructure);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the buffer object.
    /// - [`p_create_info`] is a pointer to a [`AccelerationStructureCreateInfoNV`] structure
    ///   containing parameters affecting creation of the acceleration structure.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_acceleration_structure`] is a pointer to a [`AccelerationStructureNV`] handle in which
    ///   the resulting acceleration structure object is returned.
    ///# Description
    ///Similarly to other objects in Vulkan, the acceleration structure creation
    ///merely creates an object with a specific “shape” as specified by the
    ///information in [`AccelerationStructureInfoNV`] and `compactedSize`
    ///in [`p_create_info`].
    ///Populating the data in the object after allocating and binding memory is
    ///done with [`cmd_build_acceleration_structure_nv`] and
    ///[`cmd_copy_acceleration_structure_nv`].Acceleration structure creation uses the count and
    /// type information from the
    ///geometries, but does not use the data references in the structures.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`AccelerationStructureCreateInfoNV`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_acceleration_structure`] **must**  be a valid pointer to a [`AccelerationStructureNV`]
    ///   handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureCreateInfoNV`]
    /// - [`AccelerationStructureNV`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateAccelerationStructureNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_acceleration_structure_nv<'lt>(
        self: &Unique<Device>,
        p_create_info: &AccelerationStructureCreateInfoNV<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<AccelerationStructureNV>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.create_acceleration_structure_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.create_acceleration_structure_nv())
            .unwrap_unchecked();
        let mut p_acceleration_structure = MaybeUninit::<AccelerationStructureNV>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const AccelerationStructureCreateInfoNV<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_acceleration_structure.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_acceleration_structure.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html) - Destroy an acceleration structure object
    ///# C Specifications
    ///To destroy an acceleration structure, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkDestroyAccelerationStructureNV(
    ///    VkDevice                                    device,
    ///    VkAccelerationStructureNV                   accelerationStructure,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that destroys the buffer.
    /// - [`acceleration_structure`] is the acceleration structure to destroy.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    ///# Description
    ///## Valid Usage
    /// - All submitted commands that refer to [`acceleration_structure`] **must**  have completed
    ///   execution
    /// - If [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created, a
    ///   compatible set of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`acceleration_structure`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`acceleration_structure`] is not [`crate::Handle::null`], [`acceleration_structure`]
    ///   **must**  be a valid [`AccelerationStructureNV`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`acceleration_structure`] is a valid handle, it  **must**  have been created,
    ///   allocated, or retrieved from [`device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`acceleration_structure`] **must**  be externally synchronized
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureNV`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyAccelerationStructureNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_acceleration_structure_nv<'lt>(
        self: &Unique<Device>,
        acceleration_structure: Option<AccelerationStructureNV>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.destroy_acceleration_structure_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.destroy_acceleration_structure_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            acceleration_structure.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Device {
    ///[vkGetAccelerationStructureMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html) - Get acceleration structure memory requirements
    ///# C Specifications
    ///An acceleration structure has memory requirements for the structure object
    ///itself, scratch space for the build, and scratch space for the update.Scratch space is
    /// allocated as a [`Buffer`], so for
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
    ///and
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV`
    ///the `pMemoryRequirements->alignment` and
    ///`pMemoryRequirements->memoryTypeBits` values returned by this call  **must**
    ///be filled with zero, and  **should**  be ignored by the application.To query the memory
    /// requirements, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkGetAccelerationStructureMemoryRequirementsNV(
    ///    VkDevice                                    device,
    ///    const VkAccelerationStructureMemoryRequirementsInfoNV* pInfo,
    ///    VkMemoryRequirements2KHR*                   pMemoryRequirements);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device on which the acceleration structure was created.
    /// - [`p_info`] is a pointer to a [`AccelerationStructureMemoryRequirementsInfoNV`] structure
    ///   specifying the acceleration structure to get memory requirements for.
    /// - [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2KHR`] structure in which
    ///   the requested acceleration structure memory requirements are returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_info`] **must**  be a valid pointer to a valid
    ///   [`AccelerationStructureMemoryRequirementsInfoNV`] structure
    /// - [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2KHR`]
    ///   structure
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureMemoryRequirementsInfoNV`]
    /// - [`Device`]
    /// - [`MemoryRequirements2KHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_acceleration_structure_memory_requirements_nv<'lt>(
        self: &Unique<Device>,
        p_info: &AccelerationStructureMemoryRequirementsInfoNV<'lt>,
        p_memory_requirements: Option<MemoryRequirements2<'lt>>,
    ) -> MemoryRequirements2<'lt> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.get_acceleration_structure_memory_requirements_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.get_acceleration_structure_memory_requirements_nv())
            .unwrap_unchecked();
        let mut p_memory_requirements = p_memory_requirements.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            p_info as *const AccelerationStructureMemoryRequirementsInfoNV<'lt>,
            &mut p_memory_requirements,
        );
        {
            p_memory_requirements.p_next = std::ptr::null_mut();
            p_memory_requirements
        }
    }
}
impl Device {
    ///[vkBindAccelerationStructureMemoryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html) - Bind acceleration structure memory
    ///# C Specifications
    ///To attach memory to one or more acceleration structures at a time, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkBindAccelerationStructureMemoryNV(
    ///    VkDevice                                    device,
    ///    uint32_t                                    bindInfoCount,
    ///    const VkBindAccelerationStructureMemoryInfoNV* pBindInfos);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the acceleration structures and memory.
    /// - [`bind_info_count`] is the number of elements in [`p_bind_infos`].
    /// - [`p_bind_infos`] is a pointer to an array of [`BindAccelerationStructureMemoryInfoNV`]
    ///   structures describing acceleration structures and memory to bind.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_bind_infos`] **must**  be a valid pointer to an array of [`bind_info_count`] valid
    ///   [`BindAccelerationStructureMemoryInfoNV`] structures
    /// - [`bind_info_count`] **must**  be greater than `0`
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`BindAccelerationStructureMemoryInfoNV`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkBindAccelerationStructureMemoryNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn bind_acceleration_structure_memory_nv<'lt>(
        self: &Unique<Device>,
        p_bind_infos: &[crate::extensions::nv_ray_tracing::BindAccelerationStructureMemoryInfoNV<'lt>],
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.bind_acceleration_structure_memory_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.bind_acceleration_structure_memory_nv())
            .unwrap_unchecked();
        let bind_info_count = (|len: usize| len)(p_bind_infos.len()) as _;
        let _return = _function(self.as_raw(), bind_info_count, p_bind_infos.as_ptr());
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetAccelerationStructureHandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html) - Get opaque acceleration structure handle
    ///# C Specifications
    ///To allow constructing geometry instances with device code if desired, we
    ///need to be able to query a opaque handle for an acceleration structure.
    ///This handle is a value of 8 bytes.
    ///To get this handle, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkGetAccelerationStructureHandleNV(
    ///    VkDevice                                    device,
    ///    VkAccelerationStructureNV                   accelerationStructure,
    ///    size_t                                      dataSize,
    ///    void*                                       pData);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the acceleration structures.
    /// - [`acceleration_structure`] is the acceleration structure.
    /// - [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
    /// - [`p_data`] is a pointer to a user-allocated buffer where the results will be written.
    ///# Description
    ///## Valid Usage
    /// - [`data_size`] **must**  be large enough to contain the result of the query, as described
    ///   above
    /// - [`acceleration_structure`] **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
    /// - [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
    /// - [`data_size`] **must**  be greater than `0`
    /// - [`acceleration_structure`] **must**  have been created, allocated, or retrieved from
    ///   [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureNV`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetAccelerationStructureHandleNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_acceleration_structure_handle_nv(
        self: &Unique<Device>,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.get_acceleration_structure_handle_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.get_acceleration_structure_handle_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), acceleration_structure, data_size, p_data);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkCreateRayTracingPipelinesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html) - Creates a new ray tracing pipeline object
    ///# C Specifications
    ///To create ray tracing pipelines, call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///VkResult vkCreateRayTracingPipelinesNV(
    ///    VkDevice                                    device,
    ///    VkPipelineCache                             pipelineCache,
    ///    uint32_t                                    createInfoCount,
    ///    const VkRayTracingPipelineCreateInfoNV*     pCreateInfos,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkPipeline*                                 pPipelines);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the ray tracing pipelines.
    /// - [`pipeline_cache`] is either [`crate::Handle::null`], indicating that pipeline caching is disabled, or the handle of a valid [pipeline cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-cache) object, in which case use of that cache is enabled for the duration of the command.
    /// - [`create_info_count`] is the length of the [`p_create_infos`] and [`p_pipelines`] arrays.
    /// - [`p_create_infos`] is a pointer to an array of [`RayTracingPipelineCreateInfoNV`]
    ///   structures.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_pipelines`] is a pointer to an array in which the resulting ray tracing pipeline
    ///   objects are returned.
    ///# Description
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
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`pipeline_cache`] is not [`crate::Handle::null`], [`pipeline_cache`] **must**  be a
    ///   valid [`PipelineCache`] handle
    /// - [`p_create_infos`] **must**  be a valid pointer to an array of [`create_info_count`] valid
    ///   [`RayTracingPipelineCreateInfoNV`] structures
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_pipelines`] **must**  be a valid pointer to an array of
    ///   [`create_info_count`][`Pipeline`] handles
    /// - [`create_info_count`] **must**  be greater than `0`
    /// - If [`pipeline_cache`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_INVALID_SHADER_NV`
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`Pipeline`]
    /// - [`PipelineCache`]
    /// - [`RayTracingPipelineCreateInfoNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateRayTracingPipelinesNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_ray_tracing_pipelines_nv<'lt>(
        self: &Unique<Device>,
        pipeline_cache: Option<PipelineCache>,
        p_create_infos: &[crate::extensions::nv_ray_tracing::RayTracingPipelineCreateInfoNV<'lt>],
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<SmallVec<Unique<Pipeline>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.create_ray_tracing_pipelines_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.create_ray_tracing_pipelines_nv())
            .unwrap_unchecked();
        let create_info_count = (|len: usize| len)(p_create_infos.len()) as _;
        let mut p_pipelines = SmallVec::<Pipeline>::from_elem(Default::default(), create_info_count as usize);
        let _return = _function(
            self.as_raw(),
            pipeline_cache.unwrap_or_default(),
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_pipelines.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::PIPELINE_COMPILE_REQUIRED => VulkanResult::Success(
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
impl CommandBuffer {
    ///[vkCmdCopyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) - Copy an acceleration structure
    ///# C Specifications
    ///To copy an acceleration structure call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkCmdCopyAccelerationStructureNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkAccelerationStructureNV                   dst,
    ///    VkAccelerationStructureNV                   src,
    ///    VkCopyAccelerationStructureModeKHR          mode);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`dst`] is the target acceleration structure for the copy.
    /// - [`src`] is the source acceleration structure for the copy.
    /// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations
    ///   to perform during the copy.
    ///# Description
    ///Accesses to [`src`] and [`dst`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
    ///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
    ///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
    ///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
    ///`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR` as appropriate.
    ///## Valid Usage
    /// - [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or
    ///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
    /// - The source acceleration structure [`src`] **must**  have been constructed prior to the
    ///   execution of this command
    /// - If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`] **must**  have
    ///   been constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the
    ///   build
    /// - The `buffer` used to create [`src`] **must**  be bound to device memory
    /// - The `buffer` used to create [`dst`] **must**  be bound to device memory
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
    /// - [`src`] **must**  be a valid [`AccelerationStructureNV`] handle
    /// - [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - Each of [`command_buffer`], [`dst`], and [`src`] **must**  have been created, allocated,
    ///   or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureNV`]
    /// - [`CommandBuffer`]
    /// - [`CopyAccelerationStructureModeKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdCopyAccelerationStructureNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        self: &Unique<CommandBuffer>,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_copy_acceleration_structure_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_copy_acceleration_structure_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), dst, src, mode);
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdWriteAccelerationStructuresPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) - Write acceleration structure result parameters to query results.
    ///# C Specifications
    ///To query acceleration structure size parameters call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkCmdWriteAccelerationStructuresPropertiesNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    accelerationStructureCount,
    ///    const VkAccelerationStructureNV*            pAccelerationStructures,
    ///    VkQueryType                                 queryType,
    ///    VkQueryPool                                 queryPool,
    ///    uint32_t                                    firstQuery);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`acceleration_structure_count`] is the count of acceleration structures for which to
    ///   query the property.
    /// - [`p_acceleration_structures`] is a pointer to an array of existing previously built
    ///   acceleration structures.
    /// - [`query_type`] is a [`QueryType`] value specifying the type of queries managed by the
    ///   pool.
    /// - [`query_pool`] is the query pool that will manage the results of the query.
    /// - [`first_query`] is the first query index within the query pool that will contain the
    ///   [`acceleration_structure_count`] number of results.
    ///# Description
    ///Accesses to any of the acceleration structures listed in
    ///[`p_acceleration_structures`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
    ///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
    ///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
    ///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`.
    ///## Valid Usage
    /// - [`query_pool`] **must**  have been created with a [`query_type`] matching [`query_type`]
    /// - The queries identified by [`query_pool`] and [`first_query`] **must**  be *unavailable*
    /// - `accelerationStructure` **must**  be bound completely and contiguously to a single
    ///   [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]
    /// - All acceleration structures in [`p_acceleration_structures`] **must**  have been built
    ///   prior to the execution of this command
    /// - All acceleration structures in [`p_acceleration_structures`] **must**  have been built
    ///   with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` if [`query_type`] is
    ///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
    /// - [`query_type`] **must**  be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_acceleration_structures`] **must**  be a valid pointer to an array of
    ///   [`acceleration_structure_count`] valid [`AccelerationStructureNV`] handles
    /// - [`query_type`] **must**  be a valid [`QueryType`] value
    /// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - [`acceleration_structure_count`] **must**  be greater than `0`
    /// - Each of [`command_buffer`], [`query_pool`], and the elements of
    ///   [`p_acceleration_structures`] **must**  have been created, allocated, or retrieved from
    ///   the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureNV`]
    /// - [`CommandBuffer`]
    /// - [`QueryPool`]
    /// - [`QueryType`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        self: &Unique<CommandBuffer>,
        p_acceleration_structures: &[crate::extensions::nv_ray_tracing::AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_write_acceleration_structures_properties_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_write_acceleration_structures_properties_nv())
            .unwrap_unchecked();
        let acceleration_structure_count = (|len: usize| len)(p_acceleration_structures.len()) as _;
        let _return = _function(
            self.as_raw(),
            acceleration_structure_count,
            p_acceleration_structures.as_ptr(),
            query_type,
            query_pool,
            first_query.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdBuildAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) - Build an acceleration structure
    ///# C Specifications
    ///To build an acceleration structure call:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkCmdBuildAccelerationStructureNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkAccelerationStructureInfoNV*        pInfo,
    ///    VkBuffer                                    instanceData,
    ///    VkDeviceSize                                instanceOffset,
    ///    VkBool32                                    update,
    ///    VkAccelerationStructureNV                   dst,
    ///    VkAccelerationStructureNV                   src,
    ///    VkBuffer                                    scratch,
    ///    VkDeviceSize                                scratchOffset);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`p_info`] contains the shared information for the acceleration structure’s structure.
    /// - [`instance_data`] is the buffer containing an array of
    ///   [`AccelerationStructureInstanceKHR`] structures defining acceleration structures. This
    ///   parameter  **must**  be `NULL` for bottom level acceleration structures.
    /// - [`instance_offset`] is the offset in bytes (relative to the start of [`instance_data`]) at
    ///   which the instance data is located.
    /// - [`update`] specifies whether to update the [`dst`] acceleration structure with the data in
    ///   [`src`].
    /// - [`dst`] is a pointer to the target acceleration structure for the build.
    /// - [`src`] is a pointer to an existing acceleration structure that is to be used to update
    ///   the [`dst`] acceleration structure.
    /// - [`scratch`] is the [`Buffer`] that will be used as scratch memory for the build.
    /// - [`scratch_offset`] is the offset in bytes relative to the start of [`scratch`] that will
    ///   be used as a scratch memory.
    ///# Description
    ///Accesses to [`dst`], [`src`], and [`scratch`] **must**  be
    ///[synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
    ///`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
    ///[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
    ///`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
    ///`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`.
    ///## Valid Usage
    /// - `geometryCount` **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
    /// - [`dst`] **must**  have been created with compatible [`AccelerationStructureInfoNV`] where
    ///   [`AccelerationStructureInfoNV::type_`] and [`AccelerationStructureInfoNV::flags`] are
    ///   identical, [`AccelerationStructureInfoNV::instance_count`] and
    ///   [`AccelerationStructureInfoNV::geometry_count`] for [`dst`] are greater than or equal to
    ///   the build size and each geometry in [`AccelerationStructureInfoNV::geometries`] for
    ///   [`dst`] has greater than or equal to the number of vertices, indices, and AABBs
    /// - If [`update`] is [`TRUE`], [`src`] **must**  not be [`crate::Handle::null`]
    /// - If [`update`] is [`TRUE`], [`src`] **must**  have previously been constructed with
    ///   `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV` set in
    ///   [`AccelerationStructureInfoNV::flags`] in the original build
    /// - If [`update`] is [`FALSE`], the `size` member of the [`MemoryRequirements`] structure
    ///   returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with
    ///   [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`]
    ///   and [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to
    ///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV` **must**  be less
    ///   than or equal to the size of [`scratch`] minus [`scratch_offset`]
    /// - If [`update`] is [`TRUE`], the `size` member of the [`MemoryRequirements`] structure
    ///   returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with
    ///   [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`]
    ///   and [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to
    ///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV` **must**  be less
    ///   than or equal to the size of [`scratch`] minus [`scratch_offset`]
    /// - [`scratch`] **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage
    ///   flag
    /// - If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  have been
    ///   created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
    /// - Each [`AccelerationStructureInstanceKHR::acceleration_structure_reference`] value in
    ///   [`instance_data`] **must**  be a valid device address containing a value obtained from
    ///   [`get_acceleration_structure_handle_nv`]
    /// - If [`update`] is [`TRUE`], then objects that were previously active  **must**  not be made
    ///   inactive as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
    /// -    If [`update`] is [`TRUE`], then objects that were previously inactive  **must**  not be made active as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
    /// -    If [`update`] is [`TRUE`], the [`src`] and [`dst`] objects  **must**  either be the same object or not have any [memory aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing)
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_info`] **must**  be a valid pointer to a valid [`AccelerationStructureInfoNV`]
    ///   structure
    /// - If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  be a
    ///   valid [`Buffer`] handle
    /// - [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
    /// - If [`src`] is not [`crate::Handle::null`], [`src`] **must**  be a valid
    ///   [`AccelerationStructureNV`] handle
    /// - [`scratch`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - Each of [`command_buffer`], [`dst`], [`instance_data`], [`scratch`], and [`src`] that are
    ///   valid handles of non-ignored parameters  **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`AccelerationStructureInfoNV`]
    /// - [`AccelerationStructureNV`]
    /// - [`Bool32`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBuildAccelerationStructureNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_build_acceleration_structure_nv<'lt>(
        self: &Unique<CommandBuffer>,
        p_info: &AccelerationStructureInfoNV<'lt>,
        instance_data: Option<Buffer>,
        instance_offset: DeviceSize,
        update: bool,
        dst: AccelerationStructureNV,
        src: Option<AccelerationStructureNV>,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_build_acceleration_structure_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_build_acceleration_structure_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_info as *const AccelerationStructureInfoNV<'lt>,
            instance_data.unwrap_or_default(),
            instance_offset,
            update as u8 as u32,
            dst,
            src.unwrap_or_default(),
            scratch,
            scratch_offset,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdTraceRaysNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html) - Initialize a ray tracing dispatch
    ///# C Specifications
    ///To dispatch ray tracing use:
    ///```c
    ///// Provided by VK_NV_ray_tracing
    ///void vkCmdTraceRaysNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkBuffer                                    raygenShaderBindingTableBuffer,
    ///    VkDeviceSize                                raygenShaderBindingOffset,
    ///    VkBuffer                                    missShaderBindingTableBuffer,
    ///    VkDeviceSize                                missShaderBindingOffset,
    ///    VkDeviceSize                                missShaderBindingStride,
    ///    VkBuffer                                    hitShaderBindingTableBuffer,
    ///    VkDeviceSize                                hitShaderBindingOffset,
    ///    VkDeviceSize                                hitShaderBindingStride,
    ///    VkBuffer                                    callableShaderBindingTableBuffer,
    ///    VkDeviceSize                                callableShaderBindingOffset,
    ///    VkDeviceSize                                callableShaderBindingStride,
    ///    uint32_t                                    width,
    ///    uint32_t                                    height,
    ///    uint32_t                                    depth);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`raygen_shader_binding_table_buffer`] is the buffer object that holds the shader binding
    ///   table data for the ray generation shader stage.
    /// - [`raygen_shader_binding_offset`] is the offset in bytes (relative to
    ///   [`raygen_shader_binding_table_buffer`]) of the ray generation shader being used for the
    ///   trace.
    /// - [`miss_shader_binding_table_buffer`] is the buffer object that holds the shader binding
    ///   table data for the miss shader stage.
    /// - [`miss_shader_binding_offset`] is the offset in bytes (relative to
    ///   [`miss_shader_binding_table_buffer`]) of the miss shader being used for the trace.
    /// - [`miss_shader_binding_stride`] is the size in bytes of each shader binding table record in
    ///   [`miss_shader_binding_table_buffer`].
    /// - [`hit_shader_binding_table_buffer`] is the buffer object that holds the shader binding
    ///   table data for the hit shader stages.
    /// - [`hit_shader_binding_offset`] is the offset in bytes (relative to
    ///   [`hit_shader_binding_table_buffer`]) of the hit shader group being used for the trace.
    /// - [`hit_shader_binding_stride`] is the size in bytes of each shader binding table record in
    ///   [`hit_shader_binding_table_buffer`].
    /// - [`callable_shader_binding_table_buffer`] is the buffer object that holds the shader
    ///   binding table data for the callable shader stage.
    /// - [`callable_shader_binding_offset`] is the offset in bytes (relative to
    ///   [`callable_shader_binding_table_buffer`]) of the callable shader being used for the trace.
    /// - [`callable_shader_binding_stride`] is the size in bytes of each shader binding table
    ///   record in [`callable_shader_binding_table_buffer`].
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
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// -    This command  **must**  not cause a pipeline trace ray instruction to be executed from a shader invocation with a [recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) greater than the value of `maxRecursionDepth` used to create the bound ray tracing pipeline
    /// - If [`raygen_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound
    ///   completely and contiguously to a single [`DeviceMemory`] object
    /// - [`raygen_shader_binding_offset`] **must**  be less than the size of
    ///   [`raygen_shader_binding_table_buffer`]
    /// - [`raygen_shader_binding_offset`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
    /// - If [`miss_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound
    ///   completely and contiguously to a single [`DeviceMemory`] object
    /// - [`miss_shader_binding_offset`] **must**  be less than the size of
    ///   [`miss_shader_binding_table_buffer`]
    /// - [`miss_shader_binding_offset`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
    /// - If [`hit_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound
    ///   completely and contiguously to a single [`DeviceMemory`] object
    /// - [`hit_shader_binding_offset`] **must**  be less than the size of
    ///   [`hit_shader_binding_table_buffer`]
    /// - [`hit_shader_binding_offset`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
    /// - If [`callable_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound
    ///   completely and contiguously to a single [`DeviceMemory`] object
    /// - [`callable_shader_binding_offset`] **must**  be less than the size of
    ///   [`callable_shader_binding_table_buffer`]
    /// - [`callable_shader_binding_offset`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
    /// - [`miss_shader_binding_stride`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
    /// - [`hit_shader_binding_stride`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
    /// - [`callable_shader_binding_stride`] **must**  be a multiple of
    ///   [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
    /// - [`miss_shader_binding_stride`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
    /// - [`hit_shader_binding_stride`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
    /// - [`callable_shader_binding_stride`] **must**  be less than or equal to
    ///   [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
    /// - [`width`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0]
    /// - [`height`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1]
    /// - [`depth`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`raygen_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
    /// - If [`miss_shader_binding_table_buffer`] is not [`crate::Handle::null`],
    ///   [`miss_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
    /// - If [`hit_shader_binding_table_buffer`] is not [`crate::Handle::null`],
    ///   [`hit_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
    /// - If [`callable_shader_binding_table_buffer`] is not [`crate::Handle::null`],
    ///   [`callable_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute
    ///   operations
    /// - This command  **must**  only be called outside of a render pass instance
    /// - Each of [`callable_shader_binding_table_buffer`], [`command_buffer`],
    ///   [`hit_shader_binding_table_buffer`], [`miss_shader_binding_table_buffer`], and
    ///   [`raygen_shader_binding_table_buffer`] that are valid handles of non-ignored parameters
    ///   **must**  have been created, allocated, or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`nv_ray_tracing`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdTraceRaysNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_trace_rays_nv(
        self: &Unique<CommandBuffer>,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Option<Buffer>,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Option<Buffer>,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Option<Buffer>,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: Option<u32>,
        height: Option<u32>,
        depth: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_trace_rays_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_ray_tracing()
            .and_then(|vtable| vtable.cmd_trace_rays_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer.unwrap_or_default(),
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer.unwrap_or_default(),
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer.unwrap_or_default(),
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width.unwrap_or_default() as _,
            height.unwrap_or_default() as _,
            depth.unwrap_or_default() as _,
        );
        ()
    }
}
///[VkAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureNV.html) - Opaque handle to an acceleration structure object
///# C Specifications
///Acceleration structures for the [`nv_ray_tracing`] extension are
///represented by the similar [`AccelerationStructureNV`] handles:
///```c
///// Provided by VK_NV_ray_tracing
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkAccelerationStructureNV)
///```
///# Related
/// - [`nv_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsInfoNV`]
/// - [`BindAccelerationStructureMemoryInfoNV`]
/// - [`WriteDescriptorSetAccelerationStructureNV`]
/// - [`cmd_build_acceleration_structure_nv`]
/// - [`cmd_copy_acceleration_structure_nv`]
/// - [`cmd_write_acceleration_structures_properties_nv`]
/// - [`create_acceleration_structure_nv`]
/// - [`destroy_acceleration_structure_nv`]
/// - [`get_acceleration_structure_handle_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AccelerationStructureNV(pub u64);
impl AccelerationStructureNV {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for AccelerationStructureNV {}
impl Default for AccelerationStructureNV {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for AccelerationStructureNV {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }
    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_acceleration_structure_nv(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<AccelerationStructureNV> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Device`] for functions from `VK_NV_ray_tracing`
pub struct DeviceNvRayTracingVTable {
    ///See [`FNCompileDeferredNv`] for more information.
    pub compile_deferred_nv: FNCompileDeferredNv,
    ///See [`FNCreateAccelerationStructureNv`] for more information.
    pub create_acceleration_structure_nv: FNCreateAccelerationStructureNv,
    ///See [`FNDestroyAccelerationStructureNv`] for more information.
    pub destroy_acceleration_structure_nv: FNDestroyAccelerationStructureNv,
    ///See [`FNGetAccelerationStructureMemoryRequirementsNv`] for more information.
    pub get_acceleration_structure_memory_requirements_nv: FNGetAccelerationStructureMemoryRequirementsNv,
    ///See [`FNBindAccelerationStructureMemoryNv`] for more information.
    pub bind_acceleration_structure_memory_nv: FNBindAccelerationStructureMemoryNv,
    ///See [`FNGetAccelerationStructureHandleNv`] for more information.
    pub get_acceleration_structure_handle_nv: FNGetAccelerationStructureHandleNv,
    ///See [`FNCreateRayTracingPipelinesNv`] for more information.
    pub create_ray_tracing_pipelines_nv: FNCreateRayTracingPipelinesNv,
    ///See [`FNCmdCopyAccelerationStructureNv`] for more information.
    pub cmd_copy_acceleration_structure_nv: FNCmdCopyAccelerationStructureNv,
    ///See [`FNCmdWriteAccelerationStructuresPropertiesNv`] for more information.
    pub cmd_write_acceleration_structures_properties_nv: FNCmdWriteAccelerationStructuresPropertiesNv,
    ///See [`FNCmdBuildAccelerationStructureNv`] for more information.
    pub cmd_build_acceleration_structure_nv: FNCmdBuildAccelerationStructureNv,
    ///See [`FNCmdTraceRaysNv`] for more information.
    pub cmd_trace_rays_nv: FNCmdTraceRaysNv,
    ///See [`FNGetRayTracingShaderGroupHandlesKhr`] for more information.
    pub get_ray_tracing_shader_group_handles_nv: FNGetRayTracingShaderGroupHandlesKhr,
}
impl DeviceNvRayTracingVTable {
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
            compile_deferred_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCompileDeferredNV").as_ptr()))
            },
            create_acceleration_structure_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateAccelerationStructureNV").as_ptr(),
                ))
            },
            destroy_acceleration_structure_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyAccelerationStructureNV").as_ptr(),
                ))
            },
            get_acceleration_structure_memory_requirements_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetAccelerationStructureMemoryRequirementsNV").as_ptr(),
                ))
            },
            bind_acceleration_structure_memory_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkBindAccelerationStructureMemoryNV").as_ptr(),
                ))
            },
            get_acceleration_structure_handle_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetAccelerationStructureHandleNV").as_ptr(),
                ))
            },
            create_ray_tracing_pipelines_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateRayTracingPipelinesNV").as_ptr(),
                ))
            },
            cmd_copy_acceleration_structure_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdCopyAccelerationStructureNV").as_ptr(),
                ))
            },
            cmd_write_acceleration_structures_properties_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdWriteAccelerationStructuresPropertiesNV").as_ptr(),
                ))
            },
            cmd_build_acceleration_structure_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBuildAccelerationStructureNV").as_ptr(),
                ))
            },
            cmd_trace_rays_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdTraceRaysNV").as_ptr()))
            },
            get_ray_tracing_shader_group_handles_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetRayTracingShaderGroupHandlesNV").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::compile_deferred_nv`]. See [`FNCompileDeferredNv`] for more information.
    pub fn compile_deferred_nv(&self) -> FNCompileDeferredNv {
        self.compile_deferred_nv
    }
    ///Gets [`Self::create_acceleration_structure_nv`]. See [`FNCreateAccelerationStructureNv`] for
    /// more information.
    pub fn create_acceleration_structure_nv(&self) -> FNCreateAccelerationStructureNv {
        self.create_acceleration_structure_nv
    }
    ///Gets [`Self::destroy_acceleration_structure_nv`]. See [`FNDestroyAccelerationStructureNv`]
    /// for more information.
    pub fn destroy_acceleration_structure_nv(&self) -> FNDestroyAccelerationStructureNv {
        self.destroy_acceleration_structure_nv
    }
    ///Gets [`Self::get_acceleration_structure_memory_requirements_nv`]. See
    /// [`FNGetAccelerationStructureMemoryRequirementsNv`] for more information.
    pub fn get_acceleration_structure_memory_requirements_nv(&self) -> FNGetAccelerationStructureMemoryRequirementsNv {
        self.get_acceleration_structure_memory_requirements_nv
    }
    ///Gets [`Self::bind_acceleration_structure_memory_nv`]. See
    /// [`FNBindAccelerationStructureMemoryNv`] for more information.
    pub fn bind_acceleration_structure_memory_nv(&self) -> FNBindAccelerationStructureMemoryNv {
        self.bind_acceleration_structure_memory_nv
    }
    ///Gets [`Self::get_acceleration_structure_handle_nv`]. See
    /// [`FNGetAccelerationStructureHandleNv`] for more information.
    pub fn get_acceleration_structure_handle_nv(&self) -> FNGetAccelerationStructureHandleNv {
        self.get_acceleration_structure_handle_nv
    }
    ///Gets [`Self::create_ray_tracing_pipelines_nv`]. See [`FNCreateRayTracingPipelinesNv`] for
    /// more information.
    pub fn create_ray_tracing_pipelines_nv(&self) -> FNCreateRayTracingPipelinesNv {
        self.create_ray_tracing_pipelines_nv
    }
    ///Gets [`Self::cmd_copy_acceleration_structure_nv`]. See [`FNCmdCopyAccelerationStructureNv`]
    /// for more information.
    pub fn cmd_copy_acceleration_structure_nv(&self) -> FNCmdCopyAccelerationStructureNv {
        self.cmd_copy_acceleration_structure_nv
    }
    ///Gets [`Self::cmd_write_acceleration_structures_properties_nv`]. See
    /// [`FNCmdWriteAccelerationStructuresPropertiesNv`] for more information.
    pub fn cmd_write_acceleration_structures_properties_nv(&self) -> FNCmdWriteAccelerationStructuresPropertiesNv {
        self.cmd_write_acceleration_structures_properties_nv
    }
    ///Gets [`Self::cmd_build_acceleration_structure_nv`]. See
    /// [`FNCmdBuildAccelerationStructureNv`] for more information.
    pub fn cmd_build_acceleration_structure_nv(&self) -> FNCmdBuildAccelerationStructureNv {
        self.cmd_build_acceleration_structure_nv
    }
    ///Gets [`Self::cmd_trace_rays_nv`]. See [`FNCmdTraceRaysNv`] for more information.
    pub fn cmd_trace_rays_nv(&self) -> FNCmdTraceRaysNv {
        self.cmd_trace_rays_nv
    }
    ///Gets [`Self::get_ray_tracing_shader_group_handles_nv`]. See
    /// [`FNGetRayTracingShaderGroupHandlesKhr`] for more information.
    pub fn get_ray_tracing_shader_group_handles_nv(&self) -> FNGetRayTracingShaderGroupHandlesKhr {
        self.get_ray_tracing_shader_group_handles_nv
    }
}
