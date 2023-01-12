[vkCmdNextSubpass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass.html) - Transition to the next subpass of a render pass

# C Specifications
To transition to the next subpass in the render pass instance after
recording the commands for a subpass, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdNextSubpass(
    VkCommandBuffer                             commandBuffer,
    VkSubpassContents                           contents);
```

# Parameters
- [`command_buffer`] is the command buffer in which to record the command.
- [`contents`] specifies how the commands in the next subpass will be provided, in the same fashion as the corresponding parameter of [`cmd_begin_render_pass`].

# Description
The subpass index for a render pass begins at zero when
[`cmd_begin_render_pass`] is recorded, and increments each time
[`cmd_next_subpass`] is recorded.Moving to the next subpass automatically performs any multisample resolve
operations in the subpass being ended.
End-of-subpass multisample resolves are treated as color attachment writes
for the purposes of synchronization.
This applies to resolve operations for both color and depth/stencil
attachments.
That is, they are considered to execute in the
`VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage and their
writes are synchronized with `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
Synchronization between rendering within a subpass and any resolve
operations at the end of the subpass occurs automatically, without need for
explicit dependencies or pipeline barriers.
However, if the resolve attachment is also used in a different subpass, an
explicit dependency is needed.After transitioning to the next subpass, the application  **can**  record the
commands for that subpass.
## Valid Usage
-    The current subpass index  **must**  be less than the number of subpasses in the render pass minus one
-    This command  **must**  not be recorded when transform feedback is active

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`contents`] **must**  be a valid [`SubpassContents`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`SubpassContents`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        