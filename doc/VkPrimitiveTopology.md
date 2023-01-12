[VkPrimitiveTopology](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html) - Supported primitive topologies

# C Specifications
The primitive topologies defined by [`PrimitiveTopology`] are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPrimitiveTopology {
    VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
} VkPrimitiveTopology;
```

# Description
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [separate point primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-point-lists).
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [separate line primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-line-lists).
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [connected line primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-line-strips) with consecutive lines sharing a vertex.
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [separate triangle primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-lists).
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [connected triangle primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-strips) with consecutive triangles sharing an edge.
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [connected triangle primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans) with all triangles sharing a common vertex. If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::triangle_fans`] is `VK_FALSE`, then triangle fans are not supported by the implementation, and [`VK_PRIMITIVE_TOPOLOGY`] **must**  not be used.
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [separate line primitives with adjacency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-line-lists-with-adjacency).
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [connected line primitives with adjacency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-line-strips-with-adjacency), with consecutive primitives sharing three vertices.
- [`VK_PRIMITIVE_TOPOLOGY`] specifies a series of [separate triangle primitives with adjacency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-lists-with-adjacency).
- [`VK_PRIMITIVE_TOPOLOGY`] specifies [connected triangle primitives with adjacency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-strips-with-adjacency), with consecutive triangles sharing an edge.
- [`VK_PRIMITIVE_TOPOLOGY`] specifies [separate patch primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-patch-lists).
Each primitive topology, and its construction from a list of vertices, is
described in detail below with a supporting diagram, according to the
following key:The diagrams are supported with mathematical definitions where the vertices
(v) and primitives (p) are numbered starting from 0;
v<sub>0</sub> is the first vertex in the provided data and p<sub>0</sub> is the
first primitive in the set of primitives defined by the vertices and
topology.

# Related
- [`crate::vulkan1_0`]
- [`PipelineInputAssemblyStateCreateInfo`]
- [`cmd_set_primitive_topology`]
- [`cmd_set_primitive_topology_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        