[vkGetPhysicalDeviceQueueFamilyProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html) - Reports properties of the queues of the specified physical device

# C Specifications
To query properties of queues available on a physical device, call:
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceQueueFamilyProperties(
    VkPhysicalDevice                            physicalDevice,
    uint32_t*                                   pQueueFamilyPropertyCount,
    VkQueueFamilyProperties*                    pQueueFamilyProperties);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose properties will be queried.
- [`p_queue_family_property_count`] is a pointer to an integer related to the number of queue families available or queried, as described below.
- [`p_queue_family_properties`] is either `NULL` or a pointer to an array of [`QueueFamilyProperties`] structures.

# Description
If [`p_queue_family_properties`] is `NULL`, then the number of queue families
available is returned in [`p_queue_family_property_count`].
Implementations  **must**  support at least one queue family.
Otherwise, [`p_queue_family_property_count`] **must**  point to a variable set by
the user to the number of elements in the [`p_queue_family_properties`]
array, and on return the variable is overwritten with the number of
structures actually written to [`p_queue_family_properties`].
If [`p_queue_family_property_count`] is less than the number of queue families
available, at most [`p_queue_family_property_count`] structures will be
written.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_queue_family_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_queue_family_property_count`] is not `0`, and [`p_queue_family_properties`] is not `NULL`, [`p_queue_family_properties`] **must**  be a valid pointer to an array of [`p_queue_family_property_count`][`QueueFamilyProperties`] structures

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDevice`]
- [`QueueFamilyProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        