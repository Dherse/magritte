[vkCmdEndConditionalRenderingEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) - Define the end of a conditional rendering block

# C Specifications
To end conditional rendering, call:
```c
// Provided by VK_EXT_conditional_rendering
void vkCmdEndConditionalRenderingEXT(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.

# Description
Once ended, conditional rendering becomes inactive.
## Valid Usage
-    Conditional rendering  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
-    If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) outside of a render pass instance, it  **must**  not be ended inside a render pass instance
-    If conditional rendering was made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) within a subpass it  **must**  be ended in the same subpass

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_conditional_rendering`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        