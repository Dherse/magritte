[VkSubpassContents](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html) - Specify how commands in the first subpass of a render pass are provided

# C Specifications
Possible values of [`cmd_begin_render_pass`]`::contents`, specifying
how the commands in the first subpass will be provided, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSubpassContents {
    VK_SUBPASS_CONTENTS_INLINE = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
} VkSubpassContents;
```

# Description
- [`INLINE`] specifies that the contents of the subpass will be recorded inline in the primary command buffer, and secondary command buffers  **must**  not be executed within the subpass.
- [`SECONDARY_COMMAND_BUFFERS`] specifies that the contents are recorded in secondary command buffers that will be called from the primary command buffer, and [`cmd_execute_commands`] is the only valid command on the command buffer until [`cmd_next_subpass`] or [`cmd_end_render_pass`].

# Related
- [`crate::vulkan1_0`]
- [`SubpassBeginInfo`]
- [`cmd_begin_render_pass`]
- [`cmd_next_subpass`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        