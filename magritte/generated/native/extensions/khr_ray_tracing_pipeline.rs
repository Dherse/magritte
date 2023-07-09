pub use crate::common::extensions::khr_ray_tracing_pipeline::{
    StridedDeviceAddressRegionKHR, TraceRaysIndirectCommandKHR,
};
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::native::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR;
use crate::native::{
    extensions::khr_deferred_host_operations::DeferredOperationKHR,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceAddress,
        DeviceSize, Pipeline, PipelineCache, PipelineCreateFlags, PipelineDynamicStateCreateInfo, PipelineLayout,
        PipelineShaderStageCreateInfo, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
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
    #[doc(alias = "pShaderGroupCaptureReplayHandle")]
    pub shader_group_capture_replay_handle: *const std::ffi::c_void,
}
impl Default for RayTracingShaderGroupCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::RayTracingShaderGroupCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            general_shader: unsafe { std::mem::zeroed() },
            closest_hit_shader: unsafe { std::mem::zeroed() },
            any_hit_shader: unsafe { std::mem::zeroed() },
            intersection_shader: unsafe { std::mem::zeroed() },
            shader_group_capture_replay_handle: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
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
    pub groups: *const RayTracingShaderGroupCreateInfoKHR,
    #[doc(alias = "maxPipelineRayRecursionDepth")]
    pub max_pipeline_ray_recursion_depth: u32,
    #[doc(alias = "pLibraryInfo")]
    pub library_info: *const PipelineLibraryCreateInfoKHR,
    #[doc(alias = "pLibraryInterface")]
    pub library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    #[doc(alias = "pDynamicState")]
    pub dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    pub base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    pub base_pipeline_index: i32,
}
impl Default for RayTracingPipelineCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::RayTracingPipelineCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            stage_count: unsafe { std::mem::zeroed() },
            stages: unsafe { std::mem::zeroed() },
            group_count: unsafe { std::mem::zeroed() },
            groups: unsafe { std::mem::zeroed() },
            max_pipeline_ray_recursion_depth: unsafe { std::mem::zeroed() },
            library_info: unsafe { std::mem::zeroed() },
            library_interface: unsafe { std::mem::zeroed() },
            dynamic_state: unsafe { std::mem::zeroed() },
            layout: unsafe { std::mem::zeroed() },
            base_pipeline_handle: unsafe { std::mem::zeroed() },
            base_pipeline_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "rayTracingPipeline")]
    pub ray_tracing_pipeline: Bool32,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplay")]
    pub ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplayMixed")]
    pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    #[doc(alias = "rayTracingPipelineTraceRaysIndirect")]
    pub ray_tracing_pipeline_trace_rays_indirect: Bool32,
    #[doc(alias = "rayTraversalPrimitiveCulling")]
    pub ray_traversal_primitive_culling: Bool32,
}
impl Default for PhysicalDeviceRayTracingPipelineFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRayTracingPipelineFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            ray_tracing_pipeline: unsafe { std::mem::zeroed() },
            ray_tracing_pipeline_shader_group_handle_capture_replay: unsafe { std::mem::zeroed() },
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: unsafe { std::mem::zeroed() },
            ray_tracing_pipeline_trace_rays_indirect: unsafe { std::mem::zeroed() },
            ray_traversal_primitive_culling: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderGroupHandleSize")]
    pub shader_group_handle_size: u32,
    #[doc(alias = "maxRayRecursionDepth")]
    pub max_ray_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    pub max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    pub shader_group_base_alignment: u32,
    #[doc(alias = "shaderGroupHandleCaptureReplaySize")]
    pub shader_group_handle_capture_replay_size: u32,
    #[doc(alias = "maxRayDispatchInvocationCount")]
    pub max_ray_dispatch_invocation_count: u32,
    #[doc(alias = "shaderGroupHandleAlignment")]
    pub shader_group_handle_alignment: u32,
    #[doc(alias = "maxRayHitAttributeSize")]
    pub max_ray_hit_attribute_size: u32,
}
impl Default for PhysicalDeviceRayTracingPipelinePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRayTracingPipelinePropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            shader_group_handle_size: unsafe { std::mem::zeroed() },
            max_ray_recursion_depth: unsafe { std::mem::zeroed() },
            max_shader_group_stride: unsafe { std::mem::zeroed() },
            shader_group_base_alignment: unsafe { std::mem::zeroed() },
            shader_group_handle_capture_replay_size: unsafe { std::mem::zeroed() },
            max_ray_dispatch_invocation_count: unsafe { std::mem::zeroed() },
            shader_group_handle_alignment: unsafe { std::mem::zeroed() },
            max_ray_hit_attribute_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "maxPipelineRayPayloadSize")]
    pub max_pipeline_ray_payload_size: u32,
    #[doc(alias = "maxPipelineRayHitAttributeSize")]
    pub max_pipeline_ray_hit_attribute_size: u32,
}
impl Default for RayTracingPipelineInterfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::RayTracingPipelineInterfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            max_pipeline_ray_payload_size: unsafe { std::mem::zeroed() },
            max_pipeline_ray_hit_attribute_size: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_ray_tracing_pipeline::{
    RayTracingShaderGroupTypeKHR, ShaderGroupShaderKHR, KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME,
    KHR_RAY_TRACING_PIPELINE_SPEC_VERSION,
};
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
pub type FNGetRayTracingShaderGroupHandlesKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
pub type FNGetRayTracingCaptureReplayShaderGroupHandlesKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateRayTracingPipelinesKHR")]
pub type FNCreateRayTracingPipelinesKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> VulkanResultCodes;
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
pub type FNGetRayTracingShaderGroupStackSizeKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
#[doc(alias = "vkCmdTraceRaysKHR")]
pub type FNCmdTraceRaysKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
pub type FNCmdTraceRaysIndirectKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
pub type FNCmdSetRayTracingPipelineStackSizeKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
