[VkProvokingVertexModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html) - Specify which vertex in a primitive is the provoking vertex

# C Specifications
Possible values of
[`PipelineRasterizationProvokingVertexStateCreateInfoEXT::provoking_vertex_mode`]
are:
```c
// Provided by VK_EXT_provoking_vertex
typedef enum VkProvokingVertexModeEXT {
    VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT = 0,
    VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT = 1,
} VkProvokingVertexModeEXT;
```

# Description
- [`VK_PROVOKING_VERTEX_MODE_EXT`] specifies that the provoking vertex is the first non-adjacency vertex in the list of vertices used by a primitive.
- [`VK_PROVOKING_VERTEX_MODE_EXT`] specifies that the provoking vertex is the last non-adjacency vertex in the list of vertices used by a primitive.
These modes are described more precisely in
[Primitive Topologies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topologies).

# Related
- [`ext_provoking_vertex`]
- [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        