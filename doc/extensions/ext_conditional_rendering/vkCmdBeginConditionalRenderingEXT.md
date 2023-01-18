[vkCmdBeginConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) - Define the beginning of a conditional rendering block

# C Specifications
To begin conditional rendering, call:
```c
// Provided by VK_EXT_conditional_rendering
void vkCmdBeginConditionalRenderingEXT(
    VkCommandBuffer                             commandBuffer,
    const VkConditionalRenderingBeginInfoEXT*   pConditionalRenderingBegin);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`p_conditional_rendering_begin`] is a pointer to a [`ConditionalRenderingBeginInfoEXT`] structure specifying parameters of conditional rendering.

# Description
## Valid Usage
-    Conditional rendering  **must**  not already be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_conditional_rendering_begin`] **must**  be a valid pointer to a valid [`ConditionalRenderingBeginInfoEXT`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_conditional_rendering`]
- [`CommandBuffer`]
- [`ConditionalRenderingBeginInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        