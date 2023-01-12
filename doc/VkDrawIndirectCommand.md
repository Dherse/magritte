[VkDrawIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawIndirectCommand.html) - Structure specifying a indirect drawing command

# C Specifications
The [`DrawIndirectCommand`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDrawIndirectCommand {
    uint32_t    vertexCount;
    uint32_t    instanceCount;
    uint32_t    firstVertex;
    uint32_t    firstInstance;
} VkDrawIndirectCommand;
```

# Members
- [`vertex_count`] is the number of vertices to draw.
- [`instance_count`] is the number of instances to draw.
- [`first_vertex`] is the index of the first vertex to draw.
- [`first_instance`] is the instance ID of the first instance to draw.

# Description
The members of [`DrawIndirectCommand`] have the same meaning as the
similarly named parameters of [`cmd_draw`].
## Valid Usage
-    For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained within the corresponding vertex buffer binding, as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input)
-    If the [drawIndirectFirstInstance](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-drawIndirectFirstInstance) feature is not enabled, [`first_instance`] **must**  be `0`

# Related
- [`crate::vulkan1_0`]
- [`cmd_draw_indirect`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        