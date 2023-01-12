[vkGetPhysicalDeviceQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html) - Reports properties of the queues of the specified physical device

# C Specifications
To query properties of queues available on a physical device, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceQueueFamilyProperties2(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pQueueFamilyPropertyCount,
    VkQueueFamilyProperties2*                   pQueueFamilyProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
void vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pQueueFamilyPropertyCount,
    VkQueueFamilyProperties2*                   pQueueFamilyProperties);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose properties will be queried.
- [`p_queue_family_property_count`] is a pointer to an integer related to the number of queue families available or queried, as described in [`get_physical_device_queue_family_properties`].
- [`p_queue_family_properties`] is either `NULL` or a pointer to an array of [`QueueFamilyProperties2`] structures.

# Description
[`get_physical_device_queue_family_properties2`] behaves similarly to
[`get_physical_device_queue_family_properties`], with the ability to return
extended information in a `pNext` chain of output structures.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_queue_family_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_queue_family_property_count`] is not `0`, and [`p_queue_family_properties`] is not `NULL`, [`p_queue_family_properties`] **must**  be a valid pointer to an array of [`p_queue_family_property_count`][`QueueFamilyProperties2`] structures

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`QueueFamilyProperties2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        