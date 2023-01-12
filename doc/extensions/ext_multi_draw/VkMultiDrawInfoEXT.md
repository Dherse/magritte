[VkMultiDrawInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html) - Structure specifying a multi-draw command

# C Specifications
The [`MultiDrawInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_multi_draw
typedef struct VkMultiDrawInfoEXT {
    uint32_t    firstVertex;
    uint32_t    vertexCount;
} VkMultiDrawInfoEXT;
```

# Members
- [`first_vertex`] is the first vertex to draw.
- [`vertex_count`] is the number of vertices to draw.

# Description
The members of [`MultiDrawInfoEXT`] have the same meaning as the
[`first_vertex`] and [`vertex_count`] parameters in [`cmd_draw`].

# Related
- [`ext_multi_draw`]
- [`cmd_draw_multi_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        