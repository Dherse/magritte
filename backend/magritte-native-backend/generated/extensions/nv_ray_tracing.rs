#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AabbPositionsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryInstanceFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::GeometryTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::extensions::khr_acceleration_structure::TransformMatrixKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::extensions::khr_ray_tracing_pipeline::FNGetRayTracingShaderGroupHandlesKhr;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR;
use crate::{
    cstr,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceMemory,
        DeviceSize, Format, IndexType, Pipeline, PipelineCache, PipelineCreateFlags, PipelineLayout,
        PipelineShaderStageCreateInfo, QueryPool, QueryType, StructureType, VulkanResultCodes, SHADER_UNUSED_KHR,
    },
    vulkan1_1::MemoryRequirements2,
};
use std::ffi::CStr;
///See [`GeometryFlagsKHR`]
#[doc(alias = "VkGeometryFlagsNV")]
pub type GeometryFlagsNV = GeometryFlagsKHR;
///See [`GeometryInstanceFlagsKHR`]
#[doc(alias = "VkGeometryInstanceFlagsNV")]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
///See [`BuildAccelerationStructureFlagsKHR`]
#[doc(alias = "VkBuildAccelerationStructureFlagsNV")]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
///See [`GeometryFlagBitsKHR`]
#[doc(alias = "VkGeometryFlagBitsNV")]
pub type GeometryFlagBitsNV = GeometryFlagBitsKHR;
///See [`GeometryInstanceFlagBitsKHR`]
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
pub type GeometryInstanceFlagBitsNV = GeometryInstanceFlagBitsKHR;
///See [`BuildAccelerationStructureFlagBitsKHR`]
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
pub type BuildAccelerationStructureFlagBitsNV = BuildAccelerationStructureFlagBitsKHR;
///See [`CopyAccelerationStructureModeKHR`]
#[doc(alias = "VkCopyAccelerationStructureModeNV")]
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
///See [`AccelerationStructureTypeKHR`]
#[doc(alias = "VkAccelerationStructureTypeNV")]
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
///See [`GeometryTypeKHR`]
#[doc(alias = "VkGeometryTypeNV")]
pub type GeometryTypeNV = GeometryTypeKHR;
///See [`RayTracingShaderGroupTypeKHR`]
#[doc(alias = "VkRayTracingShaderGroupTypeNV")]
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
///See [`MemoryRequirements2`]
#[doc(alias = "VkMemoryRequirements2KHR")]
pub type MemoryRequirements2KHR = MemoryRequirements2;
///See [`AabbPositionsKHR`]
#[doc(alias = "VkAabbPositionsNV")]
pub type AabbPositionsNV = AabbPositionsKHR;
///See [`TransformMatrixKHR`]
#[doc(alias = "VkTransformMatrixNV")]
pub type TransformMatrixNV = TransformMatrixKHR;
///See [`AccelerationStructureInstanceKHR`]
#[doc(alias = "VkAccelerationStructureInstanceNV")]
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
#[doc(alias = "VkRayTracingShaderGroupCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: RayTracingShaderGroupTypeKHR,
    #[doc(alias = "generalShader")]
    general_shader: u32,
    #[doc(alias = "closestHitShader")]
    closest_hit_shader: u32,
    #[doc(alias = "anyHitShader")]
    any_hit_shader: u32,
    #[doc(alias = "intersectionShader")]
    intersection_shader: u32,
}
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineCreateFlags,
    #[doc(alias = "stageCount")]
    stage_count: u32,
    #[doc(alias = "pStages")]
    stages: *const PipelineShaderStageCreateInfo,
    #[doc(alias = "groupCount")]
    group_count: u32,
    #[doc(alias = "pGroups")]
    groups: *const RayTracingShaderGroupCreateInfoNV,
    #[doc(alias = "maxRecursionDepth")]
    max_recursion_depth: u32,
    layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    base_pipeline_index: i32,
}
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryTrianglesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "vertexData")]
    vertex_data: Buffer,
    #[doc(alias = "vertexOffset")]
    vertex_offset: DeviceSize,
    #[doc(alias = "vertexCount")]
    vertex_count: u32,
    #[doc(alias = "vertexStride")]
    vertex_stride: DeviceSize,
    #[doc(alias = "vertexFormat")]
    vertex_format: Format,
    #[doc(alias = "indexData")]
    index_data: Buffer,
    #[doc(alias = "indexOffset")]
    index_offset: DeviceSize,
    #[doc(alias = "indexCount")]
    index_count: u32,
    #[doc(alias = "indexType")]
    index_type: IndexType,
    #[doc(alias = "transformData")]
    transform_data: Buffer,
    #[doc(alias = "transformOffset")]
    transform_offset: DeviceSize,
}
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryAabbNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "aabbData")]
    aabb_data: Buffer,
    #[doc(alias = "numAABBs")]
    num_aab_bs: u32,
    stride: u32,
    offset: DeviceSize,
}
#[doc(alias = "VkGeometryDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryDataNV {
    triangles: GeometryTrianglesNV,
    aabbs: GeometryAabbNV,
}
#[doc(alias = "VkGeometryNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "geometryType")]
    geometry_type: GeometryTypeKHR,
    geometry: GeometryDataNV,
    flags: GeometryFlagsKHR,
}
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: AccelerationStructureTypeNV,
    flags: BuildAccelerationStructureFlagsNV,
    #[doc(alias = "instanceCount")]
    instance_count: u32,
    #[doc(alias = "geometryCount")]
    geometry_count: u32,
    #[doc(alias = "pGeometries")]
    geometries: *const GeometryNV,
}
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "compactedSize")]
    compacted_size: DeviceSize,
    info: AccelerationStructureInfoNV,
}
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructure")]
    acceleration_structure: AccelerationStructureNV,
    memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    memory_offset: DeviceSize,
    #[doc(alias = "deviceIndexCount")]
    device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    device_indices: *const u32,
}
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureCount")]
    acceleration_structure_count: u32,
    #[doc(alias = "pAccelerationStructures")]
    acceleration_structures: *const AccelerationStructureNV,
}
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: AccelerationStructureMemoryRequirementsTypeNV,
    #[doc(alias = "accelerationStructure")]
    acceleration_structure: AccelerationStructureNV,
}
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderGroupHandleSize")]
    shader_group_handle_size: u32,
    #[doc(alias = "maxRecursionDepth")]
    max_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    shader_group_base_alignment: u32,
    #[doc(alias = "maxGeometryCount")]
    max_geometry_count: u64,
    #[doc(alias = "maxInstanceCount")]
    max_instance_count: u64,
    #[doc(alias = "maxTriangleCount")]
    max_triangle_count: u64,
    #[doc(alias = "maxDescriptorSetAccelerationStructures")]
    max_descriptor_set_acceleration_structures: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkAccelerationStructureNV")]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);
impl AccelerationStructureNV {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for AccelerationStructureNV {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_ray_tracing");
///See [`SHADER_UNUSED_KHR`]
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);
impl AccelerationStructureMemoryRequirementsTypeNV {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV")]
    pub const OBJECT: Self = Self(0);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV")]
    pub const BUILD_SCRATCH: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV")]
    pub const UPDATE_SCRATCH: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::OBJECT.bits() => Some(Self(x)),
            x if x == Self::BUILD_SCRATCH.bits() => Some(Self(x)),
            x if x == Self::UPDATE_SCRATCH.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///See [`get_ray_tracing_shader_group_handles_khr`]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesNV")]
pub type FNGetRayTracingShaderGroupHandlesNv = FNGetRayTracingShaderGroupHandlesKhr;
#[doc(alias = "vkCompileDeferredNV")]
pub type FNCompileDeferredNv =
    unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> VulkanResultCodes;
#[doc(alias = "vkCreateAccelerationStructureNV")]
pub type FNCreateAccelerationStructureNv = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureNV,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyAccelerationStructureNV")]
pub type FNDestroyAccelerationStructureNv = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkGetAccelerationStructureMemoryRequirementsNV")]
pub type FNGetAccelerationStructureMemoryRequirementsNv = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2KHR,
);
#[doc(alias = "vkBindAccelerationStructureMemoryNV")]
pub type FNBindAccelerationStructureMemoryNv = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> VulkanResultCodes;
#[doc(alias = "vkGetAccelerationStructureHandleNV")]
pub type FNGetAccelerationStructureHandleNv = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateRayTracingPipelinesNV")]
pub type FNCreateRayTracingPipelinesNv = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdCopyAccelerationStructureNV")]
pub type FNCmdCopyAccelerationStructureNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesNV")]
pub type FNCmdWriteAccelerationStructuresPropertiesNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[doc(alias = "vkCmdBuildAccelerationStructureNV")]
pub type FNCmdBuildAccelerationStructureNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const AccelerationStructureInfoNV,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);
#[doc(alias = "vkCmdTraceRaysNV")]
pub type FNCmdTraceRaysNv = unsafe extern "system" fn(
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
);
