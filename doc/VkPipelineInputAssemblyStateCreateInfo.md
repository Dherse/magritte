[VkPipelineInputAssemblyStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline input assembly state

# C Specifications
Drawing can be achieved in two modes:
- [Programmable Mesh Shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading), the mesh shader assembles primitives, or
- [Programmable Primitive Shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-shading), the input primitives are assembled
as follows.Each draw is made up of zero or more vertices and zero or more instances,
which are processed by the device and result in the assembly of primitives.
Primitives are assembled according to the `pInputAssemblyState` member
of the [`GraphicsPipelineCreateInfo`] structure, which is of type
[`PipelineInputAssemblyStateCreateInfo`]:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineInputAssemblyStateCreateInfo {
    VkStructureType                            sType;
    const void*                                pNext;
    VkPipelineInputAssemblyStateCreateFlags    flags;
    VkPrimitiveTopology                        topology;
    VkBool32                                   primitiveRestartEnable;
} VkPipelineInputAssemblyStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`topology`] is a [`PrimitiveTopology`] defining the primitive topology, as described below.
- [`primitive_restart_enable`] controls whether a special vertex index value is treated as restarting the assembly of primitives. This enable only applies to indexed draws ([`cmd_draw_indexed`], [`cmd_draw_multi_indexed_ext`], and [`cmd_draw_indexed_indirect`]), and the special index value is either 0xFFFFFFFF when the `indexType` parameter of [`cmd_bind_index_buffer`] is equal to `VK_INDEX_TYPE_UINT32`, 0xFF when `indexType` is equal to `VK_INDEX_TYPE_UINT8_EXT`, or 0xFFFF when `indexType` is equal to `VK_INDEX_TYPE_UINT16`. Primitive restart is not allowed for “list” topologies, unless one of the features [`primitiveTopologyPatchListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyPatchListRestart) (for `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST`) or [`primitiveTopologyListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyListRestart) (for all other list topologies) is enabled.

# Description
Restarting the assembly of primitives discards the most recent index values
if those elements formed an incomplete primitive, and restarts the primitive
assembly using the subsequent indices, but only assembling the immediately
following element through the end of the originally specified elements.
The primitive restart index value comparison is performed before adding the
`vertexOffset` value to the index value.
## Valid Usage
-    If [`topology`] is `VK_PRIMITIVE_TOPOLOGY_POINT_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST`, `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY` or `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY`, and [`primitive_restart_enable`] is [`TRUE`], the [`primitiveTopologyListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyListRestart) feature  **must**  be enabled
-    If [`topology`] is `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST`, and [`primitive_restart_enable`] is [`TRUE`], the [`primitiveTopologyPatchListRestart`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveTopologyPatchListRestart) feature  **must**  be enabled
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`topology`] **must**  not be any of `VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY`, `VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY`, `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY` or `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`topology`] **must**  not be `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST`
-    If the `[`VK_KHR_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::triangle_fans`] is [`FALSE`], [`topology`] **must**  not be `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`topology`] **must**  be a valid [`PrimitiveTopology`] value

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineInputAssemblyStateCreateFlags`]
- [`PrimitiveTopology`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        