[vkCreateDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDevice.html) - Create a new device instance

# C Specifications
A logical device is created as a *connection* to a physical device.
To create a logical device, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateDevice(
    VkPhysicalDevice                            physicalDevice,
    const VkDeviceCreateInfo*                   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDevice*                                   pDevice);
```

# Parameters
- [`physical_device`] **must**  be one of the device handles returned from a call to [`enumerate_physical_devices`] (see [Physical Device Enumeration](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-physical-device-enumeration)).
- [`p_create_info`] is a pointer to a [`DeviceCreateInfo`] structure containing information about how to create the device.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_device`] is a pointer to a handle in which the created [`Device`] is returned.

# Description
[`create_device`] verifies that extensions and features requested in the
`ppEnabledExtensionNames` and `pEnabledFeatures` members of
[`p_create_info`], respectively, are supported by the implementation.
If any requested extension is not supported, [`create_device`] **must** 
return `VK_ERROR_EXTENSION_NOT_PRESENT`.
If any requested feature is not supported, [`create_device`] **must**  return
`VK_ERROR_FEATURE_NOT_PRESENT`.
Support for extensions  **can**  be checked before creating a device by querying
[`enumerate_device_extension_properties`].
Support for features  **can**  similarly be checked by querying
[`get_physical_device_features`].After verifying and enabling the extensions the [`Device`] object is
created and returned to the application.Multiple logical devices  **can**  be created from the same physical device.
Logical device creation  **may**  fail due to lack of device-specific resources
(in addition to other errors).
If that occurs, [`create_device`] will return
`VK_ERROR_TOO_MANY_OBJECTS`.
## Valid Usage
-    All [required device extensions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-extensions-extensiondependencies) for each extension in the [`DeviceCreateInfo::pp_enabled_extension_names`] list  **must**  also be present in that list

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DeviceCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_device`] **must**  be a valid pointer to a [`Device`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_FEATURE_NOT_PRESENT`  - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`DeviceCreateInfo`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        