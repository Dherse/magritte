use crate::{
    extensions::{
        khr_acceleration_structure::{GeometryFlagsKHR, GeometryTypeKHR},
        khr_ray_tracing_pipeline::RayTracingShaderGroupTypeKHR,
    },
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Buffer, DeviceMemory, DeviceSize, Format, IndexType, Pipeline,
        PipelineCreateFlags, PipelineLayout, PipelineShaderStageCreateInfo, StructureType,
    },
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_ray_tracing");
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
/// - [`AccelerationStructureMemoryRequirementsTypeObjectNv`] requests the memory requirement for
///   the [`AccelerationStructureNV`] backing store.
/// - [`AccelerationStructureMemoryRequirementsTypeBuildScratchNv`] requests the memory requirement
///   for scratch space during the initial build.
/// - [`AccelerationStructureMemoryRequirementsTypeUpdateScratchNv`] requests the memory requirement
///   for scratch space during an update.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum AccelerationStructureMemoryRequirementsTypeNV {
    ///[`AccelerationStructureMemoryRequirementsTypeObjectNv`]
    ///requests the memory requirement for the [`AccelerationStructureNV`]
    ///backing store.
    AccelerationStructureMemoryRequirementsTypeObjectNv = 0,
    ///[`AccelerationStructureMemoryRequirementsTypeBuildScratchNv`]
    ///requests the memory requirement for scratch space during the initial
    ///build.
    AccelerationStructureMemoryRequirementsTypeBuildScratchNv = 1,
    ///[`AccelerationStructureMemoryRequirementsTypeUpdateScratchNv`]
    ///requests the memory requirement for scratch space during an update.
    AccelerationStructureMemoryRequirementsTypeUpdateScratchNv = 2,
}
impl const Default for AccelerationStructureMemoryRequirementsTypeNV {
    fn default() -> Self {
        AccelerationStructureMemoryRequirementsTypeObjectNv
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
///   [`RayTracingPipelineCreateInfoNV::p_stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`, and [`SHADER_UNUSED_NV`] otherwise.
/// - [`closest_hit_shader`] is the optional index of the closest hit shader from
///   [`RayTracingPipelineCreateInfoNV::p_stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
/// - [`any_hit_shader`] is the optional index of the any-hit shader from
///   [`RayTracingPipelineCreateInfoNV::p_stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
/// - [`intersection_shader`] is the index of the intersection shader from
///   [`RayTracingPipelineCreateInfoNV::p_stages`] in the group if the shader group has [`type_`] of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and [`SHADER_UNUSED_NV`]
///   otherwise.
///# Description
///Valid Usage
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`general_shader`]**must**
///   be a valid index into [`RayTracingPipelineCreateInfoNV::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_RAYGEN_BIT_NV`, `VK_SHADER_STAGE_MISS_BIT_NV`, or
///   `VK_SHADER_STAGE_CALLABLE_BIT_NV`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`closest_hit_shader`],
///   [`any_hit_shader`], and [`intersection_shader`]**must** be [`SHADER_UNUSED_NV`]
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV` then
///   [`intersection_shader`]**must** be a valid index into
///   [`RayTracingPipelineCreateInfoNV::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_INTERSECTION_BIT_NV`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` then
///   [`intersection_shader`]**must** be [`SHADER_UNUSED_NV`]
/// - [`closest_hit_shader`]**must** be either [`SHADER_UNUSED_NV`] or a valid index into
///   [`RayTracingPipelineCreateInfoNV::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`
/// - [`any_hit_shader`]**must** be either [`SHADER_UNUSED_NV`] or a valid index into
///   [`RayTracingPipelineCreateInfoNV::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_ANY_HIT_BIT_NV`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`RayTracingShaderGroupTypeKHR`] value
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`type_`] is the type of hit group specified in this structure.
    type_: RayTracingShaderGroupTypeKHR,
    ///[`general_shader`] is the index of the ray generation, miss, or
    ///callable shader from
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    general_shader: u32,
    ///[`closest_hit_shader`] is the optional index of the closest hit shader
    ///from [`RayTracingPipelineCreateInfoNV`]::`pStages` in the group
    ///if the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    closest_hit_shader: u32,
    ///[`any_hit_shader`] is the optional index of the any-hit shader from
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    any_hit_shader: u32,
    ///[`intersection_shader`] is the index of the intersection shader from
    ///[`RayTracingPipelineCreateInfoNV`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and
    ///[`SHADER_UNUSED_NV`] otherwise.
    intersection_shader: u32,
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
/// - [`stage_count`] is the number of entries in the [`p_stages`] array.
/// - [`p_stages`] is a pointer to an array of [`PipelineShaderStageCreateInfo`] structures
///   specifying the set of the shader stages to be included in the ray tracing pipeline.
/// - [`group_count`] is the number of entries in the [`p_groups`] array.
/// - [`p_groups`] is a pointer to an array of [`RayTracingShaderGroupCreateInfoNV`] structures
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
///Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).Valid Usage
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_index`] is `-1`, [`base_pipeline_handle`]**must** be a valid handle to a ray
///   tracing [`Pipeline`]
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_handle`] is [`crate::utils::Handle::null`], [`base_pipeline_index`]**must** be
///   a valid index into the calling command’s `pCreateInfos` parameter
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_index`] is not `-1`, [`base_pipeline_handle`]**must** be
///   [`crate::utils::Handle::null`]
/// - If [`flags`] contains the `VK_PIPELINE_CREATE_DERIVATIVE_BIT` flag, and
///   [`base_pipeline_handle`] is not [`crate::utils::Handle::null`],
///   [`base_pipeline_index`]**must** be `-1`
/// -    The shader code for the entry points identified by [`p_stages`], and the rest of the state identified by this structure **must** adhere to the pipeline linking rules described in the [Shader Interfaces](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#interfaces) chapter
/// - [`layout`]**must** be [consistent](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#descriptorsets-pipelinelayout-consistency)
///   with all shaders specified in [`p_stages`]
/// - The number of resources in [`layout`] accessible to each shader stage that is used by the
///   pipeline **must** be less than or equal to [`PhysicalDeviceLimits::max_per_stage_resources`]
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
/// - If the [`pipelineCreationCacheControl`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-pipelineCreationCacheControl)
///   feature is not enabled, [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` or
///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
/// - The `stage` member of at least one element of [`p_stages`]**must** be
///   `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
/// - [`max_recursion_depth`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV`]::[`max_recursion_depth`]
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`
/// - [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`
/// - [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
/// - [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
/// - [`flags`]**must** not include `VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV`
/// - [`flags`]**must** not include both `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV` and
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` at the same time
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`PipelineCreationFeedbackCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`PipelineCreateFlagBits`] values
/// - [`p_stages`]**must** be a valid pointer to an array of [`stage_count`] valid
///   [`PipelineShaderStageCreateInfo`] structures
/// - [`p_groups`]**must** be a valid pointer to an array of [`group_count`] valid
///   [`RayTracingShaderGroupCreateInfoNV`] structures
/// - [`layout`]**must** be a valid [`PipelineLayout`] handle
/// - [`stage_count`]**must** be greater than `0`
/// - [`group_count`]**must** be greater than `0`
/// - Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored
///   parameters **must** have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`Pipeline`]
/// - [`PipelineCreateFlags`]
/// - [`PipelineLayout`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`RayTracingShaderGroupCreateInfoNV`]
/// - [`StructureType`]
/// - [`CreateRayTracingPipelinesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingPipelineCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`PipelineCreateFlagBits`] specifying
    ///how the pipeline will be generated.
    flags: PipelineCreateFlags,
    ///[`stage_count`] is the number of entries in the [`p_stages`] array.
    stage_count: u32,
    ///[`p_stages`] is a pointer to an array of
    ///[`PipelineShaderStageCreateInfo`] structures specifying the set of
    ///the shader stages to be included in the ray tracing pipeline.
    p_stages: *mut PipelineShaderStageCreateInfo<'lt>,
    ///[`group_count`] is the number of entries in the [`p_groups`] array.
    group_count: u32,
    ///[`p_groups`] is a pointer to an array of
    ///[`RayTracingShaderGroupCreateInfoNV`] structures describing the set
    ///of the shader stages to be included in each shader group in the ray
    ///tracing pipeline.
    p_groups: *mut RayTracingShaderGroupCreateInfoNV<'lt>,
    ///[`max_recursion_depth`] is the [maximum
    ///recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
    max_recursion_depth: u32,
    ///[`layout`] is the description of binding locations used by both the
    ///pipeline and descriptor sets used with the pipeline.
    layout: PipelineLayout,
    ///[`base_pipeline_handle`] is a pipeline to derive from.
    base_pipeline_handle: Pipeline,
    ///[`base_pipeline_index`] is an index into the `pCreateInfos`
    ///parameter to use as a pipeline to derive from.
    base_pipeline_index: i32,
}
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
///[`index_count`].Valid Usage
/// - [`vertex_offset`]**must** be less than the size of [`vertex_data`]
/// - [`vertex_offset`]**must** be a multiple of the component size of [`vertex_format`]
/// - [`vertex_format`]**must** be one of `VK_FORMAT_R32G32B32_SFLOAT`, `VK_FORMAT_R32G32_SFLOAT`,
///   `VK_FORMAT_R16G16B16_SFLOAT`, `VK_FORMAT_R16G16_SFLOAT`, `VK_FORMAT_R16G16_SNORM`, or
///   `VK_FORMAT_R16G16B16_SNORM`
/// - [`vertex_stride`]**must** be less than or equal to 2<sup>32</sup>-1
/// - [`index_offset`]**must** be less than the size of [`index_data`]
/// - [`index_offset`]**must** be a multiple of the element size of [`index_type`]
/// - [`index_type`]**must** be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_data`]**must** be [`crate::utils::Handle::null`] if [`index_type`] is
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_data`]**must** be a valid [`Buffer`] handle if [`index_type`] is not
///   `VK_INDEX_TYPE_NONE_NV`
/// - [`index_count`]**must** be `0` if [`index_type`] is `VK_INDEX_TYPE_NONE_NV`
/// - [`transform_offset`]**must** be less than the size of [`transform_data`]
/// - [`transform_offset`]**must** be a multiple of `16`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
/// - [`p_next`]**must** be `NULL`
/// - If [`vertex_data`] is not [`crate::utils::Handle::null`], [`vertex_data`]**must** be a valid
///   [`Buffer`] handle
/// - [`vertex_format`]**must** be a valid [`Format`] value
/// - If [`index_data`] is not [`crate::utils::Handle::null`], [`index_data`]**must** be a valid
///   [`Buffer`] handle
/// - [`index_type`]**must** be a valid [`IndexType`] value
/// - If [`transform_data`] is not [`crate::utils::Handle::null`], [`transform_data`]**must** be a
///   valid [`Buffer`] handle
/// - Each of [`index_data`], [`transform_data`], and [`vertex_data`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeometryTrianglesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vertex_data`] is the buffer containing vertex data for this geometry.
    vertex_data: Buffer,
    ///[`vertex_offset`] is the offset in bytes within [`vertex_data`]
    ///containing vertex data for this geometry.
    vertex_offset: DeviceSize,
    ///[`vertex_count`] is the number of valid vertices.
    vertex_count: u32,
    ///[`vertex_stride`] is the stride in bytes between each vertex.
    vertex_stride: DeviceSize,
    ///[`vertex_format`] is a [`Format`] describing the format of each
    ///vertex element.
    vertex_format: Format,
    ///[`index_data`] is the buffer containing index data for this geometry.
    index_data: Buffer,
    ///[`index_offset`] is the offset in bytes within [`index_data`]
    ///containing index data for this geometry.
    index_offset: DeviceSize,
    ///[`index_count`] is the number of indices to include in this geometry.
    index_count: u32,
    ///[`index_type`] is a [`IndexType`] describing the format of each
    ///index.
    index_type: IndexType,
    ///[`transform_data`] is an optional buffer containing an
    ///[`TransformMatrixNV`] structure defining a transformation to be
    ///applied to this geometry.
    transform_data: Buffer,
    ///[`transform_offset`] is the offset in bytes in [`transform_data`] of
    ///the transform information described above.
    transform_offset: DeviceSize,
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
///and z values followed by the maximum x, y, and z values.Valid Usage
/// - [`offset`]**must** be less than the size of [`aabb_data`]
/// - [`offset`]**must** be a multiple of `8`
/// - [`stride`]**must** be a multiple of `8`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
/// - [`p_next`]**must** be `NULL`
/// - If [`aabb_data`] is not [`crate::utils::Handle::null`], [`aabb_data`]**must** be a valid
///   [`Buffer`] handle
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeometryAabbNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`aabb_data`] is the buffer containing axis-aligned bounding box data.
    aabb_data: Buffer,
    ///[`num_aab_bs`] is the number of AABBs in this geometry.
    num_aab_bs: u32,
    ///[`stride`] is the stride in bytes between AABBs in [`aabb_data`].
    stride: u32,
    ///[`offset`] is the offset in bytes of the first AABB in [`aabb_data`].
    offset: DeviceSize,
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
///Valid Usage (Implicit)
/// - [`triangles`]**must** be a valid [`GeometryTrianglesNV`] structure
/// - [`aabbs`]**must** be a valid [`GeometryAabbNV`] structure
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeometryDataNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`triangles`] contains triangle data if
    ///[`GeometryNV`]::`geometryType` is
    ///`VK_GEOMETRY_TYPE_TRIANGLES_NV`.
    triangles: GeometryTrianglesNV<'lt>,
    ///[`aabbs`] contains axis-aligned bounding box data if
    ///[`GeometryNV`]::`geometryType` is
    ///`VK_GEOMETRY_TYPE_AABBS_NV`.
    aabbs: GeometryAabbNV<'lt>,
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
///Valid Usage
/// - [`geometry_type`]**must** be `VK_GEOMETRY_TYPE_TRIANGLES_NV` or `VK_GEOMETRY_TYPE_AABBS_NV`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_GEOMETRY_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`geometry_type`]**must** be a valid [`GeometryTypeKHR`] value
/// - [`geometry`]**must** be a valid [`GeometryDataNV`] structure
/// - [`flags`]**must** be a valid combination of [`GeometryFlagBitsKHR`] values
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeometryNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`geometry_type`] specifies the [`GeometryTypeKHR`] which this
    ///geometry refers to.
    geometry_type: GeometryTypeKHR,
    ///[`geometry`] contains the geometry data as described in
    ///[`GeometryDataNV`].
    geometry: GeometryDataNV<'lt>,
    ///[`flags`] has [`GeometryFlagBitsKHR`] describing options for this
    ///geometry.
    flags: GeometryFlagsKHR,
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
/// - [`p_geometries`] is a pointer to an array of [`geometry_count`][`GeometryNV`] structures
///   containing the scene data being passed into the acceleration structure.
///# Description
///[`AccelerationStructureInfoNV`] contains information that is used both
///for acceleration structure creation with
///[`CreateAccelerationStructureNV`] and in combination with the actual
///geometric data to build the acceleration structure with
///[`CmdBuildAccelerationStructureNV`].Valid Usage
/// - [`geometry_count`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
/// - [`instance_count`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_instance_count`]
/// - The total number of triangles in all geometries **must** be less than or equal to
///   [`PhysicalDeviceRayTracingPropertiesNV::max_triangle_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV` then [`geometry_count`]**must**
///   be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then
///   [`instance_count`]**must** be `0`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then the `geometryType`
///   member of each geometry in [`p_geometries`]**must** be the same
/// - [`type_`]**must** not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV` bit set, then
///   it **must** not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV` bit set
/// - `scratch`**must** have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
/// - If `instanceData` is not [`crate::utils::Handle::null`], `instanceData`**must** have been
///   created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`AccelerationStructureTypeNV`] value
/// - [`flags`]**must** be a valid combination of [`BuildAccelerationStructureFlagBitsNV`] values
/// - If [`geometry_count`] is not `0`, [`p_geometries`]**must** be a valid pointer to an array of
///   [`geometry_count`] valid [`GeometryNV`] structures
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureCreateInfoNV`]
/// - [`AccelerationStructureTypeNV`]
/// - [`BuildAccelerationStructureFlagsNV`]
/// - [`GeometryNV`]
/// - [`StructureType`]
/// - [`CmdBuildAccelerationStructureNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`type_`] is a [`AccelerationStructureTypeNV`] value specifying the
    ///type of acceleration structure that will be created.
    type_: AccelerationStructureTypeNV,
    ///[`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsNV`]
    ///specifying additional parameters of the acceleration structure.
    flags: BuildAccelerationStructureFlagsNV,
    ///[`instance_count`] specifies the number of instances that will be in
    ///the new acceleration structure.
    instance_count: u32,
    ///[`geometry_count`] specifies the number of geometries that will be in
    ///the new acceleration structure.
    geometry_count: u32,
    ///[`p_geometries`] is a pointer to an array of [`geometry_count`][`GeometryNV`] structures
    /// containing the scene data being passed into the acceleration structure.
    p_geometries: *mut GeometryNV<'lt>,
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
///   [`CmdWriteAccelerationStructuresPropertiesNV`] if this acceleration structure is going to be
///   the target of a compacting copy.
/// - [`info`] is the [`AccelerationStructureInfoNV`] structure specifying further parameters of the
///   created acceleration structure.
///# Description
///Valid Usage
/// - If [`compacted_size`] is not `0` then both `info.geometryCount` and
///   `info.instanceCount`**must** be `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`info`]**must** be a valid [`AccelerationStructureInfoNV`] structure
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureInfoNV`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`CreateAccelerationStructureNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`compacted_size`] is the size from the result of
    ///[`CmdWriteAccelerationStructuresPropertiesNV`] if this acceleration
    ///structure is going to be the target of a compacting copy.
    compacted_size: DeviceSize,
    ///[`info`] is the [`AccelerationStructureInfoNV`] structure
    ///specifying further parameters of the created acceleration structure.
    info: AccelerationStructureInfoNV<'lt>,
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
/// - [`device_index_count`] is the number of elements in [`p_device_indices`].
/// - [`p_device_indices`] is a pointer to an array of device indices.
///# Description
///Valid Usage
/// - [`acceleration_structure`]**must** not already be backed by a memory object
/// - [`memory_offset`]**must** be less than the size of [`memory`]
/// - [`memory`]**must** have been allocated using one of the memory types allowed in the
///   `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetAccelerationStructureMemoryRequirementsNV`] with [`acceleration_structure`] and `type` of
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
/// - [`memory_offset`]**must** be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to
///   [`GetAccelerationStructureMemoryRequirementsNV`] with [`acceleration_structure`] and `type` of
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
/// - The `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetAccelerationStructureMemoryRequirementsNV`] with [`acceleration_structure`] and `type` of
///   `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`**must** be less than or equal
///   to the size of [`memory`] minus [`memory_offset`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`acceleration_structure`]**must** be a valid [`AccelerationStructureNV`] handle
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - If [`device_index_count`] is not `0`, [`p_device_indices`]**must** be a valid pointer to an
///   array of [`device_index_count`]`uint32_t` values
/// - Both of [`acceleration_structure`], and [`memory`]**must** have been created, allocated, or
///   retrieved from the same [`Device`]
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureNV`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`BindAccelerationStructureMemoryNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindAccelerationStructureMemoryInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`acceleration_structure`] is the acceleration structure to be attached
    ///to memory.
    acceleration_structure: AccelerationStructureNV,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of memory that is
    ///to be bound to the acceleration structure.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified
    ///acceleration structure.
    memory_offset: DeviceSize,
    ///[`device_index_count`] is the number of elements in
    ///[`p_device_indices`].
    device_index_count: u32,
    ///[`p_device_indices`] is a pointer to an array of device indices.
    p_device_indices: *mut u32,
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
/// - [`acceleration_structure_count`] is the number of elements in [`p_acceleration_structures`].
/// - [`p_acceleration_structures`] is a pointer to an array of [`AccelerationStructureNV`]
///   structures specifying the acceleration structures to update.
///# Description
///Valid Usage
/// - [`acceleration_structure_count`]**must** be equal to `descriptorCount` in the extended
///   structure
/// - Each acceleration structure in [`p_acceleration_structures`]**must** have been created with
///   `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`
/// - If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor)
///   feature is not enabled, each member of [`p_acceleration_structures`]**must** not be
///   [`crate::utils::Handle::null`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV`
/// - [`p_acceleration_structures`]**must** be a valid pointer to an array of
///   [`acceleration_structure_count`] valid or
///   [`crate::utils::Handle::null`][`AccelerationStructureNV`] handles
/// - [`acceleration_structure_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_ray_tracing`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`acceleration_structure_count`] is the number of elements in
    ///[`p_acceleration_structures`].
    acceleration_structure_count: u32,
    ///[`p_acceleration_structures`] is a pointer to an array of
    ///[`AccelerationStructureNV`] structures specifying the acceleration
    ///structures to update.
    p_acceleration_structures: *mut AccelerationStructureNV,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`AccelerationStructureMemoryRequirementsTypeNV`] value
/// - [`acceleration_structure`]**must** be a valid [`AccelerationStructureNV`] handle
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureMemoryRequirementsTypeNV`]
/// - [`AccelerationStructureNV`]
/// - [`StructureType`]
/// - [`GetAccelerationStructureMemoryRequirementsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMemoryRequirementsInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`type_`] selects the type of memory requirement being queried.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
    ///returns the memory requirements for the object itself.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV`
    ///returns the memory requirements for the scratch memory when doing a
    ///build.
    ///`VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV`
    ///returns the memory requirements for the scratch memory when doing an
    ///update.
    type_: AccelerationStructureMemoryRequirementsTypeNV,
    ///[`acceleration_structure`] is the acceleration structure to be queried
    ///for memory requirements.
    acceleration_structure: AccelerationStructureNV,
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
/// - [`shader_group_base_alignment`] is the **required** alignment in bytes for the base of the
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
///[`max_geometry_count`], [`max_instance_count`], and [`max_triangle_count`]**must** not exceed
/// 2<sup>32</sup>-1.If the [`PhysicalDeviceRayTracingPropertiesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure **must**
/// match those specified with the same
///name in [`PhysicalDeviceAccelerationStructurePropertiesKHR`] and
///[`PhysicalDeviceRayTracingPipelinePropertiesKHR`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`
///# Related
/// - [`VK_NV_ray_tracing`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_group_handle_size`] is the size in bytes of the shader header.
    shader_group_handle_size: u32,
    ///[`max_recursion_depth`] is the maximum
    ///number of levels of recursion allowed in a trace command.
    max_recursion_depth: u32,
    ///[`max_shader_group_stride`] is the maximum stride in bytes allowed
    ///between shader groups in the shader binding table.
    max_shader_group_stride: u32,
    ///[`shader_group_base_alignment`] is the **required** alignment in bytes for
    ///the base of the shader binding table.
    shader_group_base_alignment: u32,
    ///[`max_geometry_count`] is the maximum number of geometries in the bottom
    ///level acceleration structure.
    max_geometry_count: u64,
    ///[`max_instance_count`] is the maximum number of instances in the top
    ///level acceleration structure.
    max_instance_count: u64,
    ///[`max_triangle_count`] is the maximum number of triangles in all
    ///geometries in the bottom level acceleration structure.
    max_triangle_count: u64,
    ///[`max_descriptor_set_acceleration_structures`] is the maximum number of
    ///acceleration structure descriptors that are allowed in a descriptor set.
    max_descriptor_set_acceleration_structures: u32,
}
