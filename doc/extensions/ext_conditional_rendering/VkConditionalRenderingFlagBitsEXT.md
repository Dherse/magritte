[VkConditionalRenderingFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) - Specify the behavior of conditional rendering

# C Specifications
Bits which  **can**  be set in
[`cmd_begin_conditional_rendering_ext`]`::flags`, specifying the
behavior of conditional rendering, are:
```c
// Provided by VK_EXT_conditional_rendering
typedef enum VkConditionalRenderingFlagBitsEXT {
    VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = 0x00000001,
} VkConditionalRenderingFlagBitsEXT;
```

# Description
- [`INVERTED`] specifies the condition used to determine whether to discard rendering commands or not. That is, if the 32-bit predicate read from `buffer` memory at `offset` is zero, the rendering commands are not discarded, and if non zero, then they are discarded.

# Related
- [`VK_EXT_conditional_rendering`]
- [`ConditionalRenderingFlagsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        