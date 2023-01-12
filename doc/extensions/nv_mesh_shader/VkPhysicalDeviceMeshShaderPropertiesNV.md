[VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) - Structure describing mesh shading properties

# C Specifications
The [`PhysicalDeviceMeshShaderPropertiesNV`] structure is defined as:
```c
// Provided by VK_NV_mesh_shader
typedef struct VkPhysicalDeviceMeshShaderPropertiesNV {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxDrawMeshTasksCount;
    uint32_t           maxTaskWorkGroupInvocations;
    uint32_t           maxTaskWorkGroupSize[3];
    uint32_t           maxTaskTotalMemorySize;
    uint32_t           maxTaskOutputCount;
    uint32_t           maxMeshWorkGroupInvocations;
    uint32_t           maxMeshWorkGroupSize[3];
    uint32_t           maxMeshTotalMemorySize;
    uint32_t           maxMeshOutputVertices;
    uint32_t           maxMeshOutputPrimitives;
    uint32_t           maxMeshMultiviewViewCount;
    uint32_t           meshOutputPerVertexGranularity;
    uint32_t           meshOutputPerPrimitiveGranularity;
} VkPhysicalDeviceMeshShaderPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_draw_mesh_tasks_count`] is the maximum number of local workgroups that  **can**  be launched by a single draw mesh tasks command. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
- [`max_task_work_group_invocations`] is the maximum total number of task     shader invocations in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
- [`max_task_work_group_size`][3] is the maximum size of a local task     workgroup.     These three values represent the maximum local workgroup size in the X,     Y, and Z dimensions, respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`     decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
- [`max_task_total_memory_size`] is the maximum number of bytes that the task shader can use in total for shared and output memory combined.
- [`max_task_output_count`] is the maximum number of output tasks a single task shader workgroup can emit.
- [`max_mesh_work_group_invocations`] is the maximum total number of mesh     shader invocations in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
- [`max_mesh_work_group_size`][3] is the maximum size of a local mesh     workgroup.     These three values represent the maximum local workgroup size in the X,     Y, and Z dimensions, respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`     decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
- [`max_mesh_total_memory_size`] is the maximum number of bytes that the mesh shader can use in total for shared and output memory combined.
- [`max_mesh_output_vertices`] is the maximum number of vertices a mesh shader output can store.
- [`max_mesh_output_primitives`] is the maximum number of primitives a mesh shader output can store.
- [`max_mesh_multiview_view_count`] is the maximum number of multi-view views a mesh shader can use.
- [`mesh_output_per_vertex_granularity`] is the granularity with which mesh vertex outputs are allocated. The value can be used to compute the memory size used by the mesh shader, which must be less than or equal to [`max_mesh_total_memory_size`].
- [`mesh_output_per_primitive_granularity`] is the granularity with which mesh outputs qualified as per-primitive are allocated. The value can be used to compute the memory size used by the mesh shader, which must be less than or equal to [`max_mesh_total_memory_size`].

# Description
If the [`PhysicalDeviceMeshShaderPropertiesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`

# Related
- [`nv_mesh_shader`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        