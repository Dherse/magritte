[VkClearRect](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearRect.html) - Structure specifying a clear rectangle

# C Specifications
The [`ClearRect`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkClearRect {
    VkRect2D    rect;
    uint32_t    baseArrayLayer;
    uint32_t    layerCount;
} VkClearRect;
```

# Members
- [`rect`] is the two-dimensional region to be cleared.
- [`base_array_layer`] is the first layer to be cleared.
- [`layer_count`] is the number of layers to clear.

# Description
The layers [[`base_array_layer`], [`base_array_layer`] + 
[`layer_count`]) counting from the base layer of the attachment image view
are cleared.

# Related
- [`crate::vulkan1_0`]
- [`Rect2D`]
- [`cmd_clear_attachments`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        