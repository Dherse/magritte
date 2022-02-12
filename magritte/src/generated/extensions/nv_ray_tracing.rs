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
//! - An indirection table to link shader groups with acceleration structure
//!itemsThis extension adds support for the following SPIR-V extension in Vulkan:
//! - `SPV_NV_ray_tracing`
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_get_memory_requirements2`]`
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_ray_tracing]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the VK_NV_ray_tracing
//!   extension>>)
//!# New handles
//! - [`AccelerationStructureNV`]
//!# New functions & commands
//! - [`BindAccelerationStructureMemoryNV`]
//! - [`CmdBuildAccelerationStructureNV`]
//! - [`CmdCopyAccelerationStructureNV`]
//! - [`CmdTraceRaysNV`]
//! - [`CmdWriteAccelerationStructuresPropertiesNV`]
//! - [`CompileDeferredNV`]
//! - [`CreateAccelerationStructureNV`]
//! - [`CreateRayTracingPipelinesNV`]
//! - [`DestroyAccelerationStructureNV`]
//! - [`GetAccelerationStructureHandleNV`]
//! - [`GetAccelerationStructureMemoryRequirementsNV`]
//! - [`GetRayTracingShaderGroupHandlesNV`]
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
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceRayTracingPropertiesNV`]
//! - Extending [`WriteDescriptorSet`]:
//! - [`WriteDescriptorSetAccelerationStructureNV`]
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
//! - Extending [`AccelerationStructureTypeKHR`]:
//! - `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV`
//! - `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV`
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV`
//! - `VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV`
//! - Extending [`BufferUsageFlagBits`]:
//! - `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV`
//! - Extending [`BuildAccelerationStructureFlagBitsKHR`]:
//! - `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV`
//! - `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV`
//! - `VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV`
//! - `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV`
//! - `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV`
//! - Extending [`CopyAccelerationStructureModeKHR`]:
//! - `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV`
//! - `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV`
//! - Extending [`DebugReportObjectTypeEXT`]:
//! - `VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT`
//! - Extending [`DescriptorType`]:
//! - `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`
//! - Extending [`GeometryFlagBitsKHR`]:
//! - `VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV`
//! - `VK_GEOMETRY_OPAQUE_BIT_NV`
//! - Extending [`GeometryInstanceFlagBitsKHR`]:
//! - `VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV`
//! - `VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV`
//! - `VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV`
//! - `VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV`
//! - Extending [`GeometryTypeKHR`]:
//! - `VK_GEOMETRY_TYPE_AABBS_NV`
//! - `VK_GEOMETRY_TYPE_TRIANGLES_NV`
//! - Extending [`IndexType`]:
//! - `VK_INDEX_TYPE_NONE_NV`
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV`
//! - Extending [`PipelineBindPoint`]:
//! - `VK_PIPELINE_BIND_POINT_RAY_TRACING_NV`
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV`
//! - `VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV`
//! - Extending [`QueryType`]:
//! - `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
//! - Extending [`RayTracingShaderGroupTypeKHR`]:
//! - `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`
//! - `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`
//! - `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV`
//! - Extending [`ShaderStageFlagBits`]:
//! - `VK_SHADER_STAGE_ANY_HIT_BIT_NV`
//! - `VK_SHADER_STAGE_CALLABLE_BIT_NV`
//! - `VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`
//! - `VK_SHADER_STAGE_INTERSECTION_BIT_NV`
//! - `VK_SHADER_STAGE_MISS_BIT_NV`
//! - `VK_SHADER_STAGE_RAYGEN_BIT_NV`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
//! - `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
//! - `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
//! - `VK_STRUCTURE_TYPE_GEOMETRY_NV`
//! - `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`
//! - `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
//!# Known issues & F.A.Q
//!1) Are there issues?**RESOLVED**: Yes.
//!# Version History
//! - Revision 1, 2018-09-11 (Robert Stepinski, Nuno Subtil, Eric Werness)
//! - Internal revisions
//! - Revision 2, 2018-10-19 (Eric Werness)
//! - rename to VK_NV_ray_tracing, add support for callables.
//! - too many updates to list
//! - Revision 3, 2018-11-20 (Daniel Koch)
//! - update to use InstanceId instead of InstanceIndex as implemented.
//!# Other info
//! * 2018-11-20
//!*
//! - This extension requires
//![`SPV_NV_ray_tracing`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_ray_tracing.html)
//! - This extension provides API support for
//![`GL_NV_ray_tracing`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_ray_tracing.txt)
//!*
//! - Eric Werness, NVIDIA
//! - Ashwin Lele, NVIDIA
//! - Robert Stepinski, NVIDIA
//! - Nuno Subtil, NVIDIA
//! - Christoph Kubisch, NVIDIA
//! - Martin Stich, NVIDIA
//! - Daniel Koch, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Joshua Barczak, Intel
//! - Tobias Hector, AMD
//! - Henrik Rydgard, NVIDIA
//! - Pascal Gautron, NVIDIA
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
//! - [`BindAccelerationStructureMemoryNV`]
//! - [`CmdBuildAccelerationStructureNV`]
//! - [`CmdCopyAccelerationStructureNV`]
//! - [`CmdTraceRaysNV`]
//! - [`CmdWriteAccelerationStructuresPropertiesNV`]
//! - [`CompileDeferredNV`]
//! - [`CreateAccelerationStructureNV`]
//! - [`CreateRayTracingPipelinesNV`]
//! - [`DestroyAccelerationStructureNV`]
//! - [`GetAccelerationStructureHandleNV`]
//! - [`GetAccelerationStructureMemoryRequirementsNV`]
//! - [`GetRayTracingShaderGroupHandlesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::extensions::khr_ray_tracing_pipeline::SHADER_UNUSED_KHR;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_ray_tracing");
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
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
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
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT`]
///requests the memory requirement for the [`AccelerationStructureNV`]
///backing store.
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH`]
///requests the memory requirement for scratch space during the initial
///build.
/// - [`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH`]
///requests the memory requirement for scratch space during an update.
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl const Default for AccelerationStructureMemoryRequirementsTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureMemoryRequirementsTypeNV")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT"
                },
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH"
                },
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH => {
                    &"ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH"
                },
                other => unreachable!(
                    "invalid value for `AccelerationStructureMemoryRequirementsTypeNV`: {:?}",
                    other
                ),
            })
            .finish()
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT`]
    ///requests the memory requirement for the [`AccelerationStructureNV`]
    ///backing store.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH`]
    ///requests the memory requirement for scratch space during the initial
    ///build.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH: Self = Self(1);
    ///[`ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH`]
    ///requests the memory requirement for scratch space during an update.
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH: Self = Self(2);
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
