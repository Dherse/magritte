use crate::{
    extensions::khr_pipeline_library::PipelineLibraryCreateInfoKHR,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, DeviceAddress, DeviceSize, Pipeline, PipelineCreateFlags,
        PipelineDynamicStateCreateInfo, PipelineLayout, PipelineShaderStageCreateInfo, StructureType,
    },
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
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
/// - [`RayTracingShaderGroupTypeGeneralKhr`] indicates a shader group with a single
///   `VK_SHADER_STAGE_RAYGEN_BIT_KHR`, `VK_SHADER_STAGE_MISS_BIT_KHR`, or
///   `VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
/// - [`RayTracingShaderGroupTypeTrianglesHitGroupKhr`] specifies a shader group that only hits
///   triangles and **must** not contain an intersection shader, only closest hit and any-hit
///   shaders.
/// - [`RayTracingShaderGroupTypeProceduralHitGroupKhr`] specifies a shader group that only
///   intersects with custom geometry and **must** contain an intersection shader and **may**
///   contain closest hit and any-hit shaders.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum RayTracingShaderGroupTypeKHR {
    ///[`RayTracingShaderGroupTypeGeneralKhr`] indicates a shader
    ///group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`,
    ///`VK_SHADER_STAGE_MISS_BIT_KHR`, or
    ///`VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
    RayTracingShaderGroupTypeGeneralKhr = 0,
    ///[`RayTracingShaderGroupTypeTrianglesHitGroupKhr`] specifies
    ///a shader group that only hits triangles and **must** not contain an
    ///intersection shader, only closest hit and any-hit shaders.
    RayTracingShaderGroupTypeTrianglesHitGroupKhr = 1,
    ///[`RayTracingShaderGroupTypeProceduralHitGroupKhr`]
    ///specifies a shader group that only intersects with custom geometry and
    ///**must** contain an intersection shader and **may** contain closest hit and
    ///any-hit shaders.
    RayTracingShaderGroupTypeProceduralHitGroupKhr = 2,
}
impl const Default for RayTracingShaderGroupTypeKHR {
    fn default() -> Self {
        RayTracingShaderGroupTypeGeneralKhr
    }
}
impl RayTracingShaderGroupTypeKHR {
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
/// - [`ShaderGroupShaderGeneralKhr`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::general_shader`]
/// - [`ShaderGroupShaderClosestHitKhr`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::closest_hit_shader`]
/// - [`ShaderGroupShaderAnyHitKhr`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::any_hit_shader`]
/// - [`ShaderGroupShaderIntersectionKhr`] uses the shader specified in the group with
///   [`RayTracingShaderGroupCreateInfoKHR::intersection_shader`]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ShaderGroupShaderKHR {
    ///[`ShaderGroupShaderGeneralKhr`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`generalShader`
    ShaderGroupShaderGeneralKhr = 0,
    ///[`ShaderGroupShaderClosestHitKhr`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`closestHitShader`
    ShaderGroupShaderClosestHitKhr = 1,
    ///[`ShaderGroupShaderAnyHitKhr`] uses the shader specified in
    ///the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`anyHitShader`
    ShaderGroupShaderAnyHitKhr = 2,
    ///[`ShaderGroupShaderIntersectionKhr`] uses the shader specified
    ///in the group with
    ///[`RayTracingShaderGroupCreateInfoKHR`]::`intersectionShader`
    ShaderGroupShaderIntersectionKhr = 3,
}
impl const Default for ShaderGroupShaderKHR {
    fn default() -> Self {
        ShaderGroupShaderGeneralKhr
    }
}
impl ShaderGroupShaderKHR {
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
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] in the group if the shader group has [`type_`]
///   of `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
/// - [`closest_hit_shader`] is the optional index of the closest hit shader from
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] in the group if the shader group has [`type_`]
///   of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`any_hit_shader`] is the optional index of the any-hit shader from
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] in the group if the shader group has [`type_`]
///   of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`intersection_shader`] is the index of the intersection shader from
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] in the group if the shader group has [`type_`]
///   of `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`]
///   otherwise.
/// - [`p_shader_group_capture_replay_handle`] is `NULL` or a pointer to replay information for this
///   shader group. Ignored if
///   [`PhysicalDeviceRayTracingPipelineFeaturesKHR::
///   ray_tracing_pipeline_shader_group_handle_capture_replay`] is [`FALSE`].
///# Description
///Valid Usage
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`general_shader`]**must**
///   be a valid index into [`RayTracingPipelineCreateInfoKHR::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_RAYGEN_BIT_KHR`, `VK_SHADER_STAGE_MISS_BIT_KHR`, or
///   `VK_SHADER_STAGE_CALLABLE_BIT_KHR`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`closest_hit_shader`],
///   [`any_hit_shader`], and [`intersection_shader`]**must** be [`SHADER_UNUSED_KHR`]
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR` then
///   [`intersection_shader`]**must** be a valid index into
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_INTERSECTION_BIT_KHR`
/// - If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` then
///   [`intersection_shader`]**must** be [`SHADER_UNUSED_KHR`]
/// - [`closest_hit_shader`]**must** be either [`SHADER_UNUSED_KHR`] or a valid index into
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`
/// - [`any_hit_shader`]**must** be either [`SHADER_UNUSED_KHR`] or a valid index into
///   [`RayTracingPipelineCreateInfoKHR::p_stages`] referring to a shader of
///   `VK_SHADER_STAGE_ANY_HIT_BIT_KHR`
/// - If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::
///   ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then
///   [`p_shader_group_capture_replay_handle`]**must** not be provided if it has not been provided
///   on a previous call to ray tracing pipeline creation
/// - If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::
///   ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then the caller
///   **must** guarantee that no ray tracing pipeline creation commands with
///   [`p_shader_group_capture_replay_handle`] provided execute simultaneously with ray tracing
///   pipeline creation commands without [`p_shader_group_capture_replay_handle`] provided
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`RayTracingShaderGroupTypeKHR`] value
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingShaderGroupCreateInfoKHR<'lt> {
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
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    general_shader: u32,
    ///[`closest_hit_shader`] is the optional index of the closest hit shader
    ///from [`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group
    ///if the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    closest_hit_shader: u32,
    ///[`any_hit_shader`] is the optional index of the any-hit shader from
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    any_hit_shader: u32,
    ///[`intersection_shader`] is the index of the intersection shader from
    ///[`RayTracingPipelineCreateInfoKHR`]::`pStages` in the group if
    ///the shader group has [`type_`] of
    ///`VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and
    ///[`SHADER_UNUSED_KHR`] otherwise.
    intersection_shader: u32,
    ///[`p_shader_group_capture_replay_handle`] is `NULL` or a pointer to replay
    ///information for this shader group.
    ///Ignored if
    ///[`PhysicalDeviceRayTracingPipelineFeaturesKHR`]::`rayTracingPipelineShaderGroupHandleCaptureReplay`
    ///is [`FALSE`].
    p_shader_group_capture_replay_handle: *mut c_void,
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
/// - [`stage_count`] is the number of entries in the [`p_stages`] array.
/// - [`p_stages`] is a pointer to an array of [`stage_count`][`PipelineShaderStageCreateInfo`]
///   structures describing the set of the shader stages to be included in the ray tracing pipeline.
/// - [`group_count`] is the number of entries in the [`p_groups`] array.
/// - [`p_groups`] is a pointer to an array of [`group_count`][`RayTracingShaderGroupCreateInfoKHR`]
///   structures describing the set of the shader stages to be included in each shader group in the
///   ray tracing pipeline.
/// - [`max_pipeline_ray_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth)
///   of shaders executed by this pipeline.
/// - [`p_library_info`] is a pointer to a [`PipelineLibraryCreateInfoKHR`] structure defining
///   pipeline libraries to include.
/// - [`p_library_interface`] is a pointer to a [`RayTracingPipelineInterfaceCreateInfoKHR`]
///   structure defining additional information when using pipeline libraries.
/// - [`p_dynamic_state`] is a pointer to a [`PipelineDynamicStateCreateInfo`] structure, and is
///   used to indicate which properties of the pipeline state object are dynamic and **can** be
///   changed independently of the pipeline state. This **can** be `NULL`, which means no state in
///   the pipeline is considered dynamic.
/// - [`layout`] is the description of binding locations used by both the pipeline and descriptor
///   sets used with the pipeline.
/// - [`base_pipeline_handle`] is a pipeline to derive from.
/// - [`base_pipeline_index`] is an index into the `pCreateInfos` parameter to use as a pipeline to
///   derive from.
///# Description
///The parameters [`base_pipeline_handle`] and [`base_pipeline_index`] are
///described in more detail in [Pipeline
///Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives).When `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR` is specified, this pipeline
///defines a *pipeline library* which **cannot** be bound as a ray tracing
///pipeline directly.
///Instead, pipeline libraries define common shaders and shader groups which
///**can** be included in future pipeline creation.If pipeline libraries are included in
/// [`p_library_info`], shaders defined in
///those libraries are treated as if they were defined as additional entries in
///[`p_stages`], appended in the order they appear in the `pLibraries`
///array and in the [`p_stages`] array when those libraries were defined.When referencing shader
/// groups in order to obtain a shader group handle,
///groups defined in those libraries are treated as if they were defined as
///additional entries in [`p_groups`], appended in the order they appear in
///the `pLibraries` array and in the [`p_groups`] array when those
///libraries were defined.
///The shaders these groups reference are set when the pipeline library is
///created, referencing those specified in the pipeline library, not in the
///pipeline that includes it.The default stack size for a pipeline if
///`VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR` is not provided
///is computed as described in [Ray Tracing
///Pipeline Stack](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-pipeline-stack).Valid Usage
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
/// - If [`flags`] does not include `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, the `stage` member of at
///   least one element of [`p_stages`], including those implicitly added by [`p_library_info`],
///   **must** be `VK_SHADER_STAGE_RAYGEN_BIT_KHR`
/// - [`max_pipeline_ray_recursion_depth`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_recursion_depth`]
/// - If [`flags`] includes `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`, [`p_library_interface`]**must**
///   not be `NULL`
/// - If [`p_library_info`] is not `NULL` and its `libraryCount` member is greater than `0`, its
///   [`p_library_interface`] member **must** not be `NULL`
/// - Each element of `pLibraryInfo->pLibraries`**must** have been created with the value of
///   [`max_pipeline_ray_recursion_depth`] equal to that in this pipeline
/// - If [`p_library_info`] is not `NULL`, each element of its `pLibraries` member **must** have
///   been created with a [`layout`] that is compatible with the [`layout`] in this pipeline
/// - If [`p_library_info`] is not `NULL`, each element of its `pLibraries` member **must** have
///   been created with values of the `maxPipelineRayPayloadSize` and
///   `maxPipelineRayHitAttributeSize` members of [`p_library_interface`] equal to those in this
///   pipeline
/// - If [`flags`] includes
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`, each element of
///   `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, each
///   element of `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`,
///   each element of `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR`,
///   each element of `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR` bit set
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR`, each
///   element of `pLibraryInfo->pLibraries`**must** have been created with the
///   `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR` bit set
/// - If the `[`VK_KHR_pipeline_library`]` extension is not enabled, [`p_library_info`] and
///   [`p_library_interface`]**must** be `NULL`
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR`, for
///   any element of [`p_groups`] with a `type` of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `anyHitShader` of that
///   element **must** not be [`SHADER_UNUSED_KHR`]
/// - If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR`,
///   for any element of [`p_groups`] with a `type` of
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or
///   `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, the `closestHitShader` of that
///   element **must** not be [`SHADER_UNUSED_KHR`]
/// - If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling)
///   feature is not enabled, [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// - If the [`rayTraversalPrimitiveCulling`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTraversalPrimitiveCulling)
///   feature is not enabled, [`flags`]**must** not include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR`
/// - [`flags`]**must** not include both `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR` and
///   `VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR`
/// -    If [`flags`] includes `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`, [`rayTracingPipelineShaderGroupHandleCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipelineShaderGroupHandleCaptureReplay)**must** be enabled
/// - If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::
///   ray_tracing_pipeline_shader_group_handle_capture_replay`] is [`TRUE`] and the
///   `pShaderGroupCaptureReplayHandle` member of any element of [`p_groups`] is not `NULL`,
///   [`flags`]**must** include
///   `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`
/// - If [`p_library_info`] is not `NULL` and its `libraryCount` is `0`, [`stage_count`]**must** not
///   be `0`
/// - If [`p_library_info`] is not `NULL` and its `libraryCount` is `0`, [`group_count`]**must** not
///   be `0`
/// - Any element of the `pDynamicStates` member of [`p_dynamic_state`]**must** be
///   `VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`PipelineCreationFeedbackCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`PipelineCreateFlagBits`] values
/// - If [`stage_count`] is not `0`, [`p_stages`]**must** be a valid pointer to an array of
///   [`stage_count`] valid [`PipelineShaderStageCreateInfo`] structures
/// - If [`group_count`] is not `0`, [`p_groups`]**must** be a valid pointer to an array of
///   [`group_count`] valid [`RayTracingShaderGroupCreateInfoKHR`] structures
/// - If [`p_library_info`] is not `NULL`, [`p_library_info`]**must** be a valid pointer to a valid
///   [`PipelineLibraryCreateInfoKHR`] structure
/// - If [`p_library_interface`] is not `NULL`, [`p_library_interface`]**must** be a valid pointer
///   to a valid [`RayTracingPipelineInterfaceCreateInfoKHR`] structure
/// - If [`p_dynamic_state`] is not `NULL`, [`p_dynamic_state`]**must** be a valid pointer to a
///   valid [`PipelineDynamicStateCreateInfo`] structure
/// - [`layout`]**must** be a valid [`PipelineLayout`] handle
/// - Both of [`base_pipeline_handle`], and [`layout`] that are valid handles of non-ignored
///   parameters **must** have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`Pipeline`]
/// - [`PipelineCreateFlags`]
/// - [`PipelineDynamicStateCreateInfo`]
/// - [`PipelineLayout`]
/// - [`PipelineLibraryCreateInfoKHR`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`RayTracingPipelineInterfaceCreateInfoKHR`]
/// - [`RayTracingShaderGroupCreateInfoKHR`]
/// - [`StructureType`]
/// - [`CreateRayTracingPipelinesKHR`]
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
pub struct RayTracingPipelineCreateInfoKHR<'lt> {
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
    ///[`p_stages`] is a pointer to an array of [`stage_count`][`PipelineShaderStageCreateInfo`]
    /// structures describing the set of the shader stages to be included in the ray tracing
    /// pipeline.
    p_stages: *mut PipelineShaderStageCreateInfo<'lt>,
    ///[`group_count`] is the number of entries in the [`p_groups`] array.
    group_count: u32,
    ///[`p_groups`] is a pointer to an array of
    /// [`group_count`][`RayTracingShaderGroupCreateInfoKHR`] structures describing the set
    /// of the shader stages to be included in each shader group in the ray
    ///tracing pipeline.
    p_groups: *mut RayTracingShaderGroupCreateInfoKHR<'lt>,
    ///[`max_pipeline_ray_recursion_depth`] is the [maximum recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) of shaders executed by this pipeline.
    max_pipeline_ray_recursion_depth: u32,
    ///[`p_library_info`] is a pointer to a
    ///[`PipelineLibraryCreateInfoKHR`] structure defining pipeline
    ///libraries to include.
    p_library_info: *mut PipelineLibraryCreateInfoKHR<'lt>,
    ///[`p_library_interface`] is a pointer to a
    ///[`RayTracingPipelineInterfaceCreateInfoKHR`] structure defining
    ///additional information when using pipeline libraries.
    p_library_interface: *mut RayTracingPipelineInterfaceCreateInfoKHR<'lt>,
    ///[`p_dynamic_state`] is a pointer to a
    ///[`PipelineDynamicStateCreateInfo`] structure, and is used to
    ///indicate which properties of the pipeline state object are dynamic and
    ///**can** be changed independently of the pipeline state.
    ///This **can** be `NULL`, which means no state in the pipeline is considered
    ///dynamic.
    p_dynamic_state: *mut PipelineDynamicStateCreateInfo<'lt>,
    ///[`layout`] is the description of binding locations used by both the
    ///pipeline and descriptor sets used with the pipeline.
    layout: PipelineLayout,
    ///[`base_pipeline_handle`] is a pipeline to derive from.
    base_pipeline_handle: Pipeline,
    ///[`base_pipeline_index`] is an index into the `pCreateInfos`
    ///parameter to use as a pipeline to derive from.
    base_pipeline_index: i32,
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
///   **must** be specified before any non-reused handles **may** be created.
/// - [`ray_tracing_pipeline_trace_rays_indirect`] indicates whether the implementation supports
///   indirect ray tracing commands, e.g. [`CmdTraceRaysIndirectKHR`].
/// - [`ray_traversal_primitive_culling`] indicates whether the implementation supports [primitive culling during ray traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-primitive).
///If the [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayTracingPipelineFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage
/// - If [`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`TRUE`],
///   [`ray_tracing_pipeline_shader_group_handle_capture_replay`]**must** also be [`TRUE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`ray_tracing_pipeline`] indicates
    ///whether the implementation supports the ray tracing pipeline
    ///functionality.
    ///See [Ray Tracing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing).
    ray_tracing_pipeline: Bool32,
    ///[`ray_tracing_pipeline_shader_group_handle_capture_replay`] indicates whether
    ///the implementation supports saving and reusing shader group handles,
    ///e.g. for trace capture and replay.
    ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
    ///[`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] indicates
    ///whether the implementation supports reuse of shader group handles being
    ///arbitrarily mixed with creation of non-reused shader group handles.
    ///If this is [`FALSE`], all reused shader group handles **must** be
    ///specified before any non-reused handles **may** be created.
    ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
    ///[`ray_tracing_pipeline_trace_rays_indirect`] indicates whether the
    ///implementation supports indirect ray tracing commands, e.g.
    ///[`CmdTraceRaysIndirectKHR`].
    ray_tracing_pipeline_trace_rays_indirect: Bool32,
    ///[`ray_traversal_primitive_culling`] indicates whether the implementation
    ///supports [primitive culling during ray
    ///traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-primitive).
    ray_traversal_primitive_culling: Bool32,
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
/// - [`shader_group_base_alignment`] is the **required** alignment in bytes for the base of the
///   shader binding table.
/// - [`shader_group_handle_capture_replay_size`] is the number of bytes for the information
///   required to do capture and replay for shader group handles.
/// - [`max_ray_dispatch_invocation_count`] is the maximum number of ray generation shader
///   invocations which **may** be produced by a single [`CmdTraceRaysIndirectKHR`] or
///   [`CmdTraceRaysKHR`] command.
/// - [`shader_group_handle_alignment`] is the **required** alignment in bytes for each shader
///   binding table entry. The value **must** be a power of two.
/// - [`max_ray_hit_attribute_size`] is the maximum size in bytes for a ray attribute structure
///# Description
///If the [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure **must**
/// match those specified with the same
///name in [`PhysicalDeviceRayTracingPropertiesNV`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
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
pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_group_handle_size`] is the size in bytes of the shader header.
    shader_group_handle_size: u32,
    ///[`max_ray_recursion_depth`] is the
    ///maximum number of levels of ray recursion allowed in a trace command.
    max_ray_recursion_depth: u32,
    ///[`max_shader_group_stride`] is the maximum stride in bytes allowed
    ///between shader groups in the shader binding table.
    max_shader_group_stride: u32,
    ///[`shader_group_base_alignment`] is the **required** alignment in bytes for
    ///the base of the shader binding table.
    shader_group_base_alignment: u32,
    ///[`shader_group_handle_capture_replay_size`] is the number of bytes for the
    ///information required to do capture and replay for shader group handles.
    shader_group_handle_capture_replay_size: u32,
    ///[`max_ray_dispatch_invocation_count`] is the maximum number of ray
    ///generation shader invocations which **may** be produced by a single
    ///[`CmdTraceRaysIndirectKHR`] or [`CmdTraceRaysKHR`] command.
    max_ray_dispatch_invocation_count: u32,
    ///[`shader_group_handle_alignment`] is the **required** alignment in bytes for
    ///each shader binding table entry.
    ///The value **must** be a power of two.
    shader_group_handle_alignment: u32,
    ///[`max_ray_hit_attribute_size`] is the maximum size in bytes for a ray
    ///attribute structure
    max_ray_hit_attribute_size: u32,
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
/// - [`device_address`] is the device address (as returned by the [`GetBufferDeviceAddress`]
///   command) at which the region starts, or zero if the region is unused.
/// - [`stride`] is the byte stride between consecutive elements.
/// - [`size`] is the size in bytes of the region starting at [`device_address`].
///# Description
///Valid Usage
/// - If [`size`] is not zero, all addresses between [`device_address`] and [`device_address`] +
///   [`size`] - 1**must** be in the buffer device address range of the same buffer
/// - If [`size`] is not zero, [`stride`]**must** be less than or equal to the size of the buffer
///   from which [`device_address`] was queried
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`CmdTraceRaysIndirectKHR`]
/// - [`CmdTraceRaysKHR`]
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
pub struct StridedDeviceAddressRegionKHR {
    ///[`device_address`] is the device address (as returned by the
    ///[`GetBufferDeviceAddress`] command) at which the region starts, or
    ///zero if the region is unused.
    device_address: DeviceAddress,
    ///[`stride`] is the byte stride between consecutive elements.
    stride: DeviceSize,
    ///[`size`] is the size in bytes of the region starting at
    ///[`device_address`].
    size: DeviceSize,
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
///the similarly named parameters of [`CmdTraceRaysKHR`].Valid Usage
/// - [`width`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][0] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][0]
/// - [`height`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][1] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][1]
/// - [`depth`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_compute_work_group_count`][2] ×
///   [`PhysicalDeviceLimits::max_compute_work_group_size`][2]
/// - [`width`] × [`height`] × [`depth`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_dispatch_invocation_count`]
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
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
pub struct TraceRaysIndirectCommandKHR {
    ///[`width`] is the width of the ray trace query dimensions.
    width: u32,
    ///[`height`] is height of the ray trace query dimensions.
    height: u32,
    ///[`depth`] is depth of the ray trace query dimensions.
    depth: u32,
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
///scalar alignment of any of the block’s members.Valid Usage
/// - [`max_pipeline_ray_hit_attribute_size`]**must** be less than or equal to
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_hit_attribute_size`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_ray_tracing_pipeline`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RayTracingPipelineInterfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`max_pipeline_ray_payload_size`] is the maximum payload size in bytes
    ///used by any shader in the pipeline.
    max_pipeline_ray_payload_size: u32,
    ///[`max_pipeline_ray_hit_attribute_size`] is the maximum attribute structure
    ///size in bytes used by any shader in the pipeline.
    max_pipeline_ray_hit_attribute_size: u32,
}
