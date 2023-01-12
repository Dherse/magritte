[vkDeviceWaitIdle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeviceWaitIdle.html) - Wait for a device to become idle

# C Specifications
To wait on the host for the completion of outstanding queue operations for
all queues on a given logical device, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkDeviceWaitIdle(
    VkDevice                                    device);
```

# Parameters
- [`device`] is the logical device to idle.

# Description
[`device_wait_idle`] is equivalent to calling [`queue_wait_idle`] for
all queues owned by [`device`].
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle

## Host Synchronization
- Host access to all [`Queue`] objects created from [`device`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        