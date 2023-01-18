[vkCmdCuLaunchKernelNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCuLaunchKernelNVX.html) - Stub description of vkCmdCuLaunchKernelNVX

# C Specifications
There is currently no specification language written for this command.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_NVX_binary_import
void vkCmdCuLaunchKernelNVX(
    VkCommandBuffer                             commandBuffer,
    const VkCuLaunchInfoNVX*                    pLaunchInfo);
```

# Description
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_launch_info`] **must**  be a valid pointer to a valid [`CuLaunchInfoNVX`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NVX_binary_import`]
- [`CommandBuffer`]
- [`CuLaunchInfoNVX`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        