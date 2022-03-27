use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_mesh_shader");
///[VkPhysicalDeviceMeshShaderFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) - Structure describing mesh shading features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMeshShaderFeaturesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           taskShader;
///    VkBool32           meshShader;
///} VkPhysicalDeviceMeshShaderFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`task_shader`] indicates whether the task shader stage is supported.
/// - [`mesh_shader`] indicates whether the mesh shader stage is supported.
///If the [`PhysicalDeviceMeshShaderFeaturesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMeshShaderFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
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
pub struct PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`task_shader`] indicates whether the task
    ///shader stage is supported.
    task_shader: Bool32,
    ///[`mesh_shader`] indicates whether the mesh
    ///shader stage is supported.
    mesh_shader: Bool32,
}
///[VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) - Structure describing mesh shading properties
///# C Specifications
///The [`PhysicalDeviceMeshShaderPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxDrawMeshTasksCount;
///    uint32_t           maxTaskWorkGroupInvocations;
///    uint32_t           maxTaskWorkGroupSize[3];
///    uint32_t           maxTaskTotalMemorySize;
///    uint32_t           maxTaskOutputCount;
///    uint32_t           maxMeshWorkGroupInvocations;
///    uint32_t           maxMeshWorkGroupSize[3];
///    uint32_t           maxMeshTotalMemorySize;
///    uint32_t           maxMeshOutputVertices;
///    uint32_t           maxMeshOutputPrimitives;
///    uint32_t           maxMeshMultiviewViewCount;
///    uint32_t           meshOutputPerVertexGranularity;
///    uint32_t           meshOutputPerPrimitiveGranularity;
///} VkPhysicalDeviceMeshShaderPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_draw_mesh_tasks_count`] is the maximum number of local workgroups that **can** be launched by a single draw mesh tasks command. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
/// - [`max_task_work_group_invocations`] is the maximum total number of task     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration, **must** be less than or equal to this     limit.
/// - [`max_task_work_group_size`][3] is the maximum size of a local task     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules, **must** be less than or equal to the     corresponding limit.
/// - [`max_task_total_memory_size`] is the maximum number of bytes that the task shader can use in
///   total for shared and output memory combined.
/// - [`max_task_output_count`] is the maximum number of output tasks a single task shader workgroup
///   can emit.
/// - [`max_mesh_work_group_invocations`] is the maximum total number of mesh     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration, **must** be less than or equal to this     limit.
/// - [`max_mesh_work_group_size`][3] is the maximum size of a local mesh     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules, **must** be less than or equal to the     corresponding limit.
/// - [`max_mesh_total_memory_size`] is the maximum number of bytes that the mesh shader can use in
///   total for shared and output memory combined.
/// - [`max_mesh_output_vertices`] is the maximum number of vertices a mesh shader output can store.
/// - [`max_mesh_output_primitives`] is the maximum number of primitives a mesh shader output can
///   store.
/// - [`max_mesh_multiview_view_count`] is the maximum number of multi-view views a mesh shader can
///   use.
/// - [`mesh_output_per_vertex_granularity`] is the granularity with which mesh vertex outputs are
///   allocated. The value can be used to compute the memory size used by the mesh shader, which
///   must be less than or equal to [`max_mesh_total_memory_size`].
/// - [`mesh_output_per_primitive_granularity`] is the granularity with which mesh outputs qualified
///   as per-primitive are allocated. The value can be used to compute the memory size used by the
///   mesh shader, which must be less than or equal to [`max_mesh_total_memory_size`].
///# Description
///If the [`PhysicalDeviceMeshShaderPropertiesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
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
pub struct PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_draw_mesh_tasks_count`] is the maximum number of local workgroups
    ///that **can** be launched by a single draw mesh tasks command.
    ///See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
    max_draw_mesh_tasks_count: u32,
    ///[`max_task_work_group_invocations`] is the maximum total number of task
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration, **must** be less than or equal to this
    ///    limit.
    max_task_work_group_invocations: u32,
    ///[`max_task_work_group_size`][3] is the maximum size of a local task
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules, **must** be less than or equal to the
    ///    corresponding limit.
    max_task_work_group_size: [u32; 3],
    ///[`max_task_total_memory_size`] is the maximum number of bytes that the
    ///task shader can use in total for shared and output memory combined.
    max_task_total_memory_size: u32,
    ///[`max_task_output_count`] is the maximum number of output tasks a single
    ///task shader workgroup can emit.
    max_task_output_count: u32,
    ///[`max_mesh_work_group_invocations`] is the maximum total number of mesh
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration, **must** be less than or equal to this
    ///    limit.
    max_mesh_work_group_invocations: u32,
    ///[`max_mesh_work_group_size`][3] is the maximum size of a local mesh
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules, **must** be less than or equal to the
    ///    corresponding limit.
    max_mesh_work_group_size: [u32; 3],
    ///[`max_mesh_total_memory_size`] is the maximum number of bytes that the
    ///mesh shader can use in total for shared and output memory combined.
    max_mesh_total_memory_size: u32,
    ///[`max_mesh_output_vertices`] is the maximum number of vertices a mesh
    ///shader output can store.
    max_mesh_output_vertices: u32,
    ///[`max_mesh_output_primitives`] is the maximum number of primitives a mesh
    ///shader output can store.
    max_mesh_output_primitives: u32,
    ///[`max_mesh_multiview_view_count`] is the maximum number of multi-view
    ///views a mesh shader can use.
    max_mesh_multiview_view_count: u32,
    ///[`mesh_output_per_vertex_granularity`] is the granularity with which mesh
    ///vertex outputs are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    mesh_output_per_vertex_granularity: u32,
    ///[`mesh_output_per_primitive_granularity`] is the granularity with which
    ///mesh outputs qualified as per-primitive are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    mesh_output_per_primitive_granularity: u32,
}
///[VkDrawMeshTasksIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) - Structure specifying a mesh tasks draw indirect command
///# C Specifications
///The [`DrawMeshTasksIndirectCommandNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkDrawMeshTasksIndirectCommandNV {
///    uint32_t    taskCount;
///    uint32_t    firstTask;
///} VkDrawMeshTasksIndirectCommandNV;
///```
///# Members
/// - [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z
///   dimension are implicitly set to one.
/// - [`first_task`] is the X component of the first workgroup ID.
///# Description
///The members of [`DrawMeshTasksIndirectCommandNV`] have the same meaning
///as the similarly named parameters of [`CmdDrawMeshTasksNV`].Valid Usage
/// - [`task_count`]**must** be less than or equal to
///   [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`CmdDrawMeshTasksIndirectNV`]
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
pub struct DrawMeshTasksIndirectCommandNV {
    ///[`task_count`] is the number of local workgroups to dispatch in the X
    ///dimension.
    ///Y and Z dimension are implicitly set to one.
    task_count: u32,
    ///[`first_task`] is the X component of the first workgroup ID.
    first_task: u32,
}
