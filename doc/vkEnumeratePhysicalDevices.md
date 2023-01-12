[vkEnumeratePhysicalDevices](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDevices.html) - Enumerates the physical devices accessible to a Vulkan instance

# C Specifications
To retrieve a list of physical device objects representing the physical
devices installed in the system, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkEnumeratePhysicalDevices(
    VkInstance                                  instance,
    uint32_t*                                   pPhysicalDeviceCount,
    VkPhysicalDevice*                           pPhysicalDevices);
```

# Parameters
- [`instance`] is a handle to a Vulkan instance previously created with [`create_instance`].
- [`p_physical_device_count`] is a pointer to an integer related to the number of physical devices available or queried, as described below.
- [`p_physical_devices`] is either `NULL` or a pointer to an array of [`PhysicalDevice`] handles.

# Description
If [`p_physical_devices`] is `NULL`, then the number of physical devices
available is returned in [`p_physical_device_count`].
Otherwise, [`p_physical_device_count`] **must**  point to a variable set by the
user to the number of elements in the [`p_physical_devices`] array, and on
return the variable is overwritten with the number of handles actually
written to [`p_physical_devices`].
If [`p_physical_device_count`] is less than the number of physical devices
available, at most [`p_physical_device_count`] structures will be written,
and `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the available physical devices were returned.
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_physical_device_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_physical_device_count`] is not `0`, and [`p_physical_devices`] is not `NULL`, [`p_physical_devices`] **must**  be a valid pointer to an array of [`p_physical_device_count`][`PhysicalDevice`] handles

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`crate::vulkan1_0`]
- [`Instance`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        