[vkGetDeviceQueue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceQueue.html) - Get a queue handle from a device

# C Specifications
To retrieve a handle to a [`Queue`] object, call:
```c
// Provided by VK_VERSION_1_0
void vkGetDeviceQueue(
    VkDevice                                    device,
    uint32_t                                    queueFamilyIndex,
    uint32_t                                    queueIndex,
    VkQueue*                                    pQueue);
```

# Parameters
- [`device`] is the logical device that owns the queue.
- [`queue_family_index`] is the index of the queue family to which the queue belongs.
- [`queue_index`] is the index within this queue family of the queue to retrieve.
- [`p_queue`] is a pointer to a [`Queue`] object that will be filled with the handle for the requested queue.

# Description
[`get_device_queue`] **must**  only be used to get queues that were created
with the `flags` parameter of [`DeviceQueueCreateInfo`] set to zero.
To get queues that were created with a non-zero `flags` parameter use
[`get_device_queue2`].
## Valid Usage
-  [`queue_family_index`] **must**  be one of the queue family indices specified when [`device`] was created, via the [`DeviceQueueCreateInfo`] structure
-  [`queue_index`] **must**  be less than the value of [`DeviceQueueCreateInfo::queue_count`] for the queue family indicated by [`queue_family_index`] when [`device`] was created
-  [`DeviceQueueCreateInfo::flags`] **must**  have been set to zero when [`device`] was created

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_queue`] **must**  be a valid pointer to a [`Queue`] handle

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        