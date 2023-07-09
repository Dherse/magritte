#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AabbPositionsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::GeometryFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::GeometryFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::GeometryInstanceFlagBitsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::GeometryTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::native::extensions::khr_acceleration_structure::TransformMatrixKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::native::extensions::khr_ray_tracing_pipeline::FNGetRayTracingShaderGroupHandlesKhr;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::native::extensions::khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR;
use crate::native::{
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceMemory,
        DeviceSize, Format, IndexType, Pipeline, PipelineCache, PipelineCreateFlags, PipelineLayout,
        PipelineShaderStageCreateInfo, QueryPool, QueryType, StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
};
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
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: RayTracingShaderGroupTypeKHR,
    #[doc(alias = "generalShader")]
    pub general_shader: u32,
    #[doc(alias = "closestHitShader")]
    pub closest_hit_shader: u32,
    #[doc(alias = "anyHitShader")]
    pub any_hit_shader: u32,
    #[doc(alias = "intersectionShader")]
    pub intersection_shader: u32,
}
impl Default for RayTracingShaderGroupCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::RayTracingShaderGroupCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            general_shader: unsafe { std::mem::zeroed() },
            closest_hit_shader: unsafe { std::mem::zeroed() },
            any_hit_shader: unsafe { std::mem::zeroed() },
            intersection_shader: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRayTracingPipelineCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineCreateFlags,
    #[doc(alias = "stageCount")]
    pub stage_count: u32,
    #[doc(alias = "pStages")]
    pub stages: *const PipelineShaderStageCreateInfo,
    #[doc(alias = "groupCount")]
    pub group_count: u32,
    #[doc(alias = "pGroups")]
    pub groups: *const RayTracingShaderGroupCreateInfoNV,
    #[doc(alias = "maxRecursionDepth")]
    pub max_recursion_depth: u32,
    pub layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::RayTracingPipelineCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            stage_count: unsafe { std::mem::zeroed() },
            stages: unsafe { std::mem::zeroed() },
            group_count: unsafe { std::mem::zeroed() },
            groups: unsafe { std::mem::zeroed() },
            max_recursion_depth: unsafe { std::mem::zeroed() },
            layout: unsafe { std::mem::zeroed() },
            base_pipeline_handle: unsafe { std::mem::zeroed() },
            base_pipeline_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeometryTrianglesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryTrianglesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "vertexData")]
    pub vertex_data: Buffer,
    #[doc(alias = "vertexOffset")]
    pub vertex_offset: DeviceSize,
    #[doc(alias = "vertexCount")]
    pub vertex_count: u32,
    #[doc(alias = "vertexStride")]
    pub vertex_stride: DeviceSize,
    #[doc(alias = "vertexFormat")]
    pub vertex_format: Format,
    #[doc(alias = "indexData")]
    pub index_data: Buffer,
    #[doc(alias = "indexOffset")]
    pub index_offset: DeviceSize,
    #[doc(alias = "indexCount")]
    pub index_count: u32,
    #[doc(alias = "indexType")]
    pub index_type: IndexType,
    #[doc(alias = "transformData")]
    pub transform_data: Buffer,
    #[doc(alias = "transformOffset")]
    pub transform_offset: DeviceSize,
}
impl Default for GeometryTrianglesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GeometryTrianglesNv,
            p_next: unsafe { std::mem::zeroed() },
            vertex_data: unsafe { std::mem::zeroed() },
            vertex_offset: unsafe { std::mem::zeroed() },
            vertex_count: unsafe { std::mem::zeroed() },
            vertex_stride: unsafe { std::mem::zeroed() },
            vertex_format: unsafe { std::mem::zeroed() },
            index_data: unsafe { std::mem::zeroed() },
            index_offset: unsafe { std::mem::zeroed() },
            index_count: unsafe { std::mem::zeroed() },
            index_type: unsafe { std::mem::zeroed() },
            transform_data: unsafe { std::mem::zeroed() },
            transform_offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeometryAABBNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryAabbNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "aabbData")]
    pub aabb_data: Buffer,
    #[doc(alias = "numAABBs")]
    pub num_aab_bs: u32,
    pub stride: u32,
    pub offset: DeviceSize,
}
impl Default for GeometryAabbNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GeometryAabbNv,
            p_next: unsafe { std::mem::zeroed() },
            aabb_data: unsafe { std::mem::zeroed() },
            num_aab_bs: unsafe { std::mem::zeroed() },
            stride: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeometryDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryDataNV {
    pub triangles: GeometryTrianglesNV,
    pub aabbs: GeometryAabbNV,
}
impl Default for GeometryDataNV {
    fn default() -> Self {
        Self {
            triangles: unsafe { std::mem::zeroed() },
            aabbs: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkGeometryNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct GeometryNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "geometryType")]
    pub geometry_type: GeometryTypeKHR,
    pub geometry: GeometryDataNV,
    pub flags: GeometryFlagsKHR,
}
impl Default for GeometryNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::GeometryNv,
            p_next: unsafe { std::mem::zeroed() },
            geometry_type: unsafe { std::mem::zeroed() },
            geometry: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: AccelerationStructureTypeNV,
    pub flags: BuildAccelerationStructureFlagsNV,
    #[doc(alias = "instanceCount")]
    pub instance_count: u32,
    #[doc(alias = "geometryCount")]
    pub geometry_count: u32,
    #[doc(alias = "pGeometries")]
    pub geometries: *const GeometryNV,
}
impl Default for AccelerationStructureInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            instance_count: unsafe { std::mem::zeroed() },
            geometry_count: unsafe { std::mem::zeroed() },
            geometries: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "compactedSize")]
    pub compacted_size: DeviceSize,
    pub info: AccelerationStructureInfoNV,
}
impl Default for AccelerationStructureCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            compacted_size: unsafe { std::mem::zeroed() },
            info: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindAccelerationStructureMemoryInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureNV,
    pub memory: DeviceMemory,
    #[doc(alias = "memoryOffset")]
    pub memory_offset: DeviceSize,
    #[doc(alias = "deviceIndexCount")]
    pub device_index_count: u32,
    #[doc(alias = "pDeviceIndices")]
    pub device_indices: *const u32,
}
impl Default for BindAccelerationStructureMemoryInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindAccelerationStructureMemoryInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            memory_offset: unsafe { std::mem::zeroed() },
            device_index_count: unsafe { std::mem::zeroed() },
            device_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureCount")]
    pub acceleration_structure_count: u32,
    #[doc(alias = "pAccelerationStructures")]
    pub acceleration_structures: *const AccelerationStructureNV,
}
impl Default for WriteDescriptorSetAccelerationStructureNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::WriteDescriptorSetAccelerationStructureNv,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure_count: unsafe { std::mem::zeroed() },
            acceleration_structures: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureMemoryRequirementsInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: AccelerationStructureMemoryRequirementsTypeNV,
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureNV,
}
impl Default for AccelerationStructureMemoryRequirementsInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureMemoryRequirementsInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            acceleration_structure: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderGroupHandleSize")]
    pub shader_group_handle_size: u32,
    #[doc(alias = "maxRecursionDepth")]
    pub max_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    pub max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    pub shader_group_base_alignment: u32,
    #[doc(alias = "maxGeometryCount")]
    pub max_geometry_count: u64,
    #[doc(alias = "maxInstanceCount")]
    pub max_instance_count: u64,
    #[doc(alias = "maxTriangleCount")]
    pub max_triangle_count: u64,
    #[doc(alias = "maxDescriptorSetAccelerationStructures")]
    pub max_descriptor_set_acceleration_structures: u32,
}
impl Default for PhysicalDeviceRayTracingPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRayTracingPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            shader_group_handle_size: unsafe { std::mem::zeroed() },
            max_recursion_depth: unsafe { std::mem::zeroed() },
            max_shader_group_stride: unsafe { std::mem::zeroed() },
            shader_group_base_alignment: unsafe { std::mem::zeroed() },
            max_geometry_count: unsafe { std::mem::zeroed() },
            max_instance_count: unsafe { std::mem::zeroed() },
            max_triangle_count: unsafe { std::mem::zeroed() },
            max_descriptor_set_acceleration_structures: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkAccelerationStructureNV")]
#[repr(transparent)]
pub struct AccelerationStructureNV(u64);
impl AccelerationStructureNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for AccelerationStructureNV {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::nv_ray_tracing::{
    AccelerationStructureMemoryRequirementsTypeNV, NV_RAY_TRACING_EXTENSION_NAME, NV_RAY_TRACING_SPEC_VERSION,
    SHADER_UNUSED_NV,
};
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
