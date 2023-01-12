[vkGetPhysicalDeviceExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html) - Function for querying external fence handle capabilities.

# C Specifications
Fences  **may**  support import and export of their
[payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-payloads) to external handles.
To query the external handle types supported by fences, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceExternalFenceProperties(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalFenceInfo*    pExternalFenceInfo,
    VkExternalFenceProperties*                  pExternalFenceProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_external_fence_capabilities
void vkGetPhysicalDeviceExternalFencePropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalFenceInfo*    pExternalFenceInfo,
    VkExternalFenceProperties*                  pExternalFenceProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the fence capabilities.
- [`p_external_fence_info`] is a pointer to a [`PhysicalDeviceExternalFenceInfo`] structure describing the parameters that would be consumed by [`create_fence`].
- [`p_external_fence_properties`] is a pointer to a [`ExternalFenceProperties`] structure in which capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_external_fence_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceExternalFenceInfo`] structure
-  [`p_external_fence_properties`] **must**  be a valid pointer to a [`ExternalFenceProperties`] structure

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceProperties`]
- [`PhysicalDevice`]
- [`PhysicalDeviceExternalFenceInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        