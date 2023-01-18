[VkMultiDrawIndexedInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) - Structure specifying a multi-draw command

# C Specifications
The [`MultiDrawIndexedInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_multi_draw
typedef struct VkMultiDrawIndexedInfoEXT {
    uint32_t    firstIndex;
    uint32_t    indexCount;
    int32_t     vertexOffset;
} VkMultiDrawIndexedInfoEXT;
```

# Members
- [`first_index`] is the first index to draw.
- [`index_count`] is the number of vertices to draw.
- [`vertex_offset`] is the value added to the vertex index before indexing into the vertex buffer for indexed multidraws.

# Description
The [`first_index`], [`index_count`], and [`vertex_offset`] members of
[`MultiDrawIndexedInfoEXT`] have the same meaning as the
[`first_index`], [`index_count`], and [`vertex_offset`] parameters,
respectively, of [`cmd_draw_indexed`].

# Related
- [`VK_EXT_multi_draw`]
- [`cmd_draw_multi_indexed_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        