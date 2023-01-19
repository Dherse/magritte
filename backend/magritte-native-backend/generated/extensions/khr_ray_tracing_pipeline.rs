//!# [VK_KHR_ray_tracing_pipeline](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_pipeline.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VK_KHR_ray_tracing_pipeline.md")]
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR;
use crate::{
    cstr,
    extensions::khr_deferred_host_operations::DeferredOperationKHR,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, DeviceAddress,
        DeviceSize, Pipeline, PipelineCache, PipelineCreateFlags, PipelineDynamicStateCreateInfo, PipelineLayout,
        PipelineShaderStageCreateInfo, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkRayTracingShaderGroupCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkRayTracingShaderGroupCreateInfoKHR.md")]
#[doc(alias = "VkRayTracingShaderGroupCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR {
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
    #[doc(alias = "pShaderGroupCaptureReplayHandle")]
    shader_group_capture_replay_handle: *const std::ffi::c_void,
}
///# [VkRayTracingPipelineCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkRayTracingPipelineCreateInfoKHR.md")]
#[doc(alias = "VkRayTracingPipelineCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoKHR {
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
    groups: *const RayTracingShaderGroupCreateInfoKHR,
    #[doc(alias = "maxPipelineRayRecursionDepth")]
    max_pipeline_ray_recursion_depth: u32,
    #[doc(alias = "pLibraryInfo")]
    library_info: *const PipelineLibraryCreateInfoKHR,
    #[doc(alias = "pLibraryInterface")]
    library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR,
    #[doc(alias = "pDynamicState")]
    dynamic_state: *const PipelineDynamicStateCreateInfo,
    layout: PipelineLayout,
    #[doc(alias = "basePipelineHandle")]
    base_pipeline_handle: Pipeline,
    #[doc(alias = "basePipelineIndex")]
    base_pipeline_index: i32,
}
///# [VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelineFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "rayTracingPipeline")]
    ray_tracing_pipeline: Bool32,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplay")]
    ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    #[doc(alias = "rayTracingPipelineShaderGroupHandleCaptureReplayMixed")]
    ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    #[doc(alias = "rayTracingPipelineTraceRaysIndirect")]
    ray_tracing_pipeline_trace_rays_indirect: Bool32,
    #[doc(alias = "rayTraversalPrimitiveCulling")]
    ray_traversal_primitive_culling: Bool32,
}
///# [VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.md")]
#[doc(alias = "VkPhysicalDeviceRayTracingPipelinePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderGroupHandleSize")]
    shader_group_handle_size: u32,
    #[doc(alias = "maxRayRecursionDepth")]
    max_ray_recursion_depth: u32,
    #[doc(alias = "maxShaderGroupStride")]
    max_shader_group_stride: u32,
    #[doc(alias = "shaderGroupBaseAlignment")]
    shader_group_base_alignment: u32,
    #[doc(alias = "shaderGroupHandleCaptureReplaySize")]
    shader_group_handle_capture_replay_size: u32,
    #[doc(alias = "maxRayDispatchInvocationCount")]
    max_ray_dispatch_invocation_count: u32,
    #[doc(alias = "shaderGroupHandleAlignment")]
    shader_group_handle_alignment: u32,
    #[doc(alias = "maxRayHitAttributeSize")]
    max_ray_hit_attribute_size: u32,
}
///# [VkStridedDeviceAddressRegionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkStridedDeviceAddressRegionKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkStridedDeviceAddressRegionKHR.md")]
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
    stride: DeviceSize,
    size: DeviceSize,
}
///# [VkTraceRaysIndirectCommandKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkTraceRaysIndirectCommandKHR.md")]
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    width: u32,
    height: u32,
    depth: u32,
}
///# [VkRayTracingPipelineInterfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkRayTracingPipelineInterfaceCreateInfoKHR.md")]
#[doc(alias = "VkRayTracingPipelineInterfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "maxPipelineRayPayloadSize")]
    max_pipeline_ray_payload_size: u32,
    #[doc(alias = "maxPipelineRayHitAttributeSize")]
    max_pipeline_ray_hit_attribute_size: u32,
}
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_ray_tracing_pipeline");
///# [VkRayTracingShaderGroupTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkRayTracingShaderGroupTypeKHR.md")]
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct RayTracingShaderGroupTypeKHR(i32);
impl RayTracingShaderGroupTypeKHR {
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR")]
    pub const GENERAL: Self = Self(0);
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR")]
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR")]
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const GENERAL_NV: Self = Self::GENERAL;
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP;
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP;
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
            x if x == Self::GENERAL.bits() => Some(Self(x)),
            x if x == Self::TRIANGLES_HIT_GROUP.bits() => Some(Self(x)),
            x if x == Self::PROCEDURAL_HIT_GROUP.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkShaderGroupShaderKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/VkShaderGroupShaderKHR.md")]
#[doc(alias = "VkShaderGroupShaderKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ShaderGroupShaderKHR(i32);
impl ShaderGroupShaderKHR {
    #[doc(alias = "VK_SHADER_GROUP_SHADER_GENERAL_KHR")]
    pub const GENERAL: Self = Self(0);
    #[doc(alias = "VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR")]
    pub const CLOSEST_HIT: Self = Self(1);
    #[doc(alias = "VK_SHADER_GROUP_SHADER_ANY_HIT_KHR")]
    pub const ANY_HIT: Self = Self(2);
    #[doc(alias = "VK_SHADER_GROUP_SHADER_INTERSECTION_KHR")]
    pub const INTERSECTION: Self = Self(3);
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
            x if x == Self::GENERAL.bits() => Some(Self(x)),
            x if x == Self::CLOSEST_HIT.bits() => Some(Self(x)),
            x if x == Self::ANY_HIT.bits() => Some(Self(x)),
            x if x == Self::INTERSECTION.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkGetRayTracingShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkGetRayTracingShaderGroupHandlesKHR.md")]
#[doc(alias = "vkGetRayTracingShaderGroupHandlesKHR")]
pub type FNGetRayTracingShaderGroupHandlesKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
///# [vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.md")]
#[doc(alias = "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")]
pub type FNGetRayTracingCaptureReplayShaderGroupHandlesKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
) -> VulkanResultCodes;
///# [vkCreateRayTracingPipelinesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkCreateRayTracingPipelinesKHR.md")]
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
///# [vkGetRayTracingShaderGroupStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkGetRayTracingShaderGroupStackSizeKHR.md")]
#[doc(alias = "vkGetRayTracingShaderGroupStackSizeKHR")]
pub type FNGetRayTracingShaderGroupStackSizeKhr = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
///# [vkCmdTraceRaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkCmdTraceRaysKHR.md")]
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
///# [vkCmdTraceRaysIndirectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkCmdTraceRaysIndirectKHR.md")]
#[doc(alias = "vkCmdTraceRaysIndirectKHR")]
pub type FNCmdTraceRaysIndirectKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
///# [vkCmdSetRayTracingPipelineStackSizeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_tracing_pipeline/vkCmdSetRayTracingPipelineStackSizeKHR.md")]
#[doc(alias = "vkCmdSetRayTracingPipelineStackSizeKHR")]
pub type FNCmdSetRayTracingPipelineStackSizeKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
