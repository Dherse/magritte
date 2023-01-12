[VkDrawIndexedIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawIndexedIndirectCommand.html) - Structure specifying a indexed indirect drawing command

# C Specifications
The [`DrawIndexedIndirectCommand`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDrawIndexedIndirectCommand {
    uint32_t    indexCount;
    uint32_t    instanceCount;
    uint32_t    firstIndex;
    int32_t     vertexOffset;
    uint32_t    firstInstance;
} VkDrawIndexedIndirectCommand;
```

# Members
- [`index_count`] is the number of vertices to draw.
- [`instance_count`] is the number of instances to draw.
- [`first_index`] is the base index within the index buffer.
- [`vertex_offset`] is the value added to the vertex index before indexing into the vertex buffer.
- [`first_instance`] is the instance ID of the first instance to draw.

# Description
The members of [`DrawIndexedIndirectCommand`] have the same meaning as
the similarly named parameters of [`cmd_draw_indexed`].
## Valid Usage
-    For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained within the corresponding vertex buffer binding, as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input)
-  (`indexSize` × ([`first_index`] +  [`index_count`]) +  `offset`) **must**  be less than or equal to the size of the bound index buffer, with `indexSize` being based on the type specified by `indexType`, where the index buffer, `indexType`, and `offset` are specified via [`cmd_bind_index_buffer`]
-    If the [drawIndirectFirstInstance](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-drawIndirectFirstInstance) feature is not enabled, [`first_instance`] **must**  be `0`

# Related
- [`crate::vulkan1_0`]
- [`cmd_draw_indexed_indirect`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        