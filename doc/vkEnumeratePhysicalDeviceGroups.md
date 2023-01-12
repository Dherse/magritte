[vkEnumeratePhysicalDeviceGroups](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html) - Enumerates groups of physical devices that can be used to create a single logical device

# C Specifications
To retrieve a list of the device groups present in the system, call:
```c
// Provided by VK_VERSION_1_1
VkResult vkEnumeratePhysicalDeviceGroups(
    VkInstance                                  instance,
    uint32_t*                                   pPhysicalDeviceGroupCount,
    VkPhysicalDeviceGroupProperties*            pPhysicalDeviceGroupProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_device_group_creation
VkResult vkEnumeratePhysicalDeviceGroupsKHR(
    VkInstance                                  instance,
    uint32_t*                                   pPhysicalDeviceGroupCount,
    VkPhysicalDeviceGroupProperties*            pPhysicalDeviceGroupProperties);
```

# Parameters
- [`instance`] is a handle to a Vulkan instance previously created with [`create_instance`].
- [`p_physical_device_group_count`] is a pointer to an integer related to the number of device groups available or queried, as described below.
- [`p_physical_device_group_properties`] is either `NULL` or a pointer to an array of [`PhysicalDeviceGroupProperties`] structures.

# Description
If [`p_physical_device_group_properties`] is `NULL`, then the number of device
groups available is returned in [`p_physical_device_group_count`].
Otherwise, [`p_physical_device_group_count`] **must**  point to a variable set by
the user to the number of elements in the
[`p_physical_device_group_properties`] array, and on return the variable is
overwritten with the number of structures actually written to
[`p_physical_device_group_properties`].
If [`p_physical_device_group_count`] is less than the number of device groups
available, at most [`p_physical_device_group_count`] structures will be
written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available device groups were
returned.Every physical device  **must**  be in exactly one device group.
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_physical_device_group_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_physical_device_group_count`] is not `0`, and [`p_physical_device_group_properties`] is not `NULL`, [`p_physical_device_group_properties`] **must**  be a valid pointer to an array of [`p_physical_device_group_count`][`PhysicalDeviceGroupProperties`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`crate::vulkan1_1`]
- [`Instance`]
- [`PhysicalDeviceGroupProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        