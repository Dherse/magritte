[vkCmdSetColorWriteEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html) - Enable or disable writes to a color attachment dynamically for a command buffer

# C Specifications
To [dynamically enable or disable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) writes to a
color attachment, call:
```c
// Provided by VK_EXT_color_write_enable
void                                    vkCmdSetColorWriteEnableEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    attachmentCount,
    const VkBool32*                             pColorWriteEnables);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`attachment_count`] is the number of [`Bool32`] elements in [`p_color_write_enables`].
- [`p_color_write_enables`] is a pointer to an array of per target attachment boolean values specifying whether color writes are enabled for the given attachment.

# Description
This command sets the color write enables for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineColorWriteCreateInfoEXT`]::[`p_color_write_enables`] values
used to create the currently active pipeline.
## Valid Usage
-    The [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable) feature  **must**  be enabled
-  [`attachment_count`] **must**  be less than or equal to the `maxColorAttachments` member of [`PhysicalDeviceLimits`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_color_write_enables`] **must**  be a valid pointer to an array of [`attachment_count`][`Bool32`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`attachment_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_color_write_enable`]
- [`Bool32`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        