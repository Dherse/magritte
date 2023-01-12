[vkGetDeviceQueue2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue2.html) - Get a queue handle from a device

# C Specifications
To retrieve a handle to a [`Queue`] object with specific
[VkDeviceQueueCreateFlags]() creation flags, call:
```c
// Provided by VK_VERSION_1_1
void vkGetDeviceQueue2(
    VkDevice                                    device,
    const VkDeviceQueueInfo2*                   pQueueInfo,
    VkQueue*                                    pQueue);
```

# Parameters
- [`device`] is the logical device that owns the queue.
- [`p_queue_info`] is a pointer to a [`DeviceQueueInfo2`] structure, describing parameters of the device queue to be retrieved.
- [`p_queue`] is a pointer to a [`Queue`] object that will be filled with the handle for the requested queue.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_queue_info`] **must**  be a valid pointer to a valid [`DeviceQueueInfo2`] structure
-  [`p_queue`] **must**  be a valid pointer to a [`Queue`] handle

# Related
- [`crate::vulkan1_1`]
- [`Device`]
- [`DeviceQueueInfo2`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        