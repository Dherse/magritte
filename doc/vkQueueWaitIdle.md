[vkQueueWaitIdle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueWaitIdle.html) - Wait for a queue to become idle

# C Specifications
To wait on the host for the completion of outstanding queue operations for a
given queue, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkQueueWaitIdle(
    VkQueue                                     queue);
```

# Parameters
- [`queue`] is the queue on which to wait.

# Description
[`queue_wait_idle`] is equivalent to having submitted a valid fence to
every previously executed [queue submission
command](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission) that accepts a fence, then waiting for all of those fences to
signal using [`wait_for_fences`] with an infinite timeout and
`waitAll` set to [`TRUE`].
## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle

## Host Synchronization
- Host access to [`queue`] **must**  be externally synchronized

## Command Properties
## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        