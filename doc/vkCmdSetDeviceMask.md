[vkCmdSetDeviceMask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDeviceMask.html) - Modify device mask of a command buffer

# C Specifications
To update the current device mask of a command buffer, call:
```c
// Provided by VK_VERSION_1_1
void vkCmdSetDeviceMask(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    deviceMask);
```
or the equivalent command
```c
// Provided by VK_KHR_device_group
void vkCmdSetDeviceMaskKHR(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    deviceMask);
```

# Parameters
- [`command_buffer`] is command buffer whose current device mask is modified.
- [`device_mask`] is the new value of the current device mask.

# Description
[`device_mask`] is used to filter out subsequent commands from executing on
all physical devices whose bit indices are not set in the mask, except
commands beginning a render pass instance, commands transitioning to the
next subpass in the render pass instance, and commands ending a render pass
instance, which always execute on the set of physical devices whose bit
indices are included in the [`device_mask`] member of the
[`DeviceGroupRenderPassBeginInfo`] structure passed to the command
beginning the corresponding render pass instance.
## Valid Usage
-  [`device_mask`] **must**  be a valid device mask value
-  [`device_mask`] **must**  not be zero
-  [`device_mask`] **must**  not include any set bits that were not in the [`DeviceGroupCommandBufferBeginInfo`]::[`device_mask`] value when the command buffer began recording
-    If [`cmd_set_device_mask`] is called inside a render pass instance, [`device_mask`] **must**  not include any set bits that were not in the [`DeviceGroupRenderPassBeginInfo`]::[`device_mask`] value when the render pass instance began recording

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, compute, or transfer operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_1`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        