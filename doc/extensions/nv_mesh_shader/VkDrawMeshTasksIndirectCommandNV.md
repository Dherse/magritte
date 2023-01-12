[VkDrawMeshTasksIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) - Structure specifying a mesh tasks draw indirect command

# C Specifications
The [`DrawMeshTasksIndirectCommandNV`] structure is defined as:
```c
// Provided by VK_NV_mesh_shader
typedef struct VkDrawMeshTasksIndirectCommandNV {
    uint32_t    taskCount;
    uint32_t    firstTask;
} VkDrawMeshTasksIndirectCommandNV;
```

# Members
- [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z dimension are implicitly set to one.
- [`first_task`] is the X component of the first workgroup ID.

# Description
The members of [`DrawMeshTasksIndirectCommandNV`] have the same meaning
as the similarly named parameters of [`cmd_draw_mesh_tasks_nv`].
## Valid Usage
-  [`task_count`] **must**  be less than or equal to [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]

# Related
- [`nv_mesh_shader`]
- [`cmd_draw_mesh_tasks_indirect_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        