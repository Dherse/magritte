[vkGetPhysicalDeviceExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html) - Function for querying external semaphore handle capabilities.

# C Specifications
Semaphores  **may**  support import and export of their
[payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-payloads) to external handles.
To query the external handle types supported by semaphores, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceExternalSemaphoreProperties(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalSemaphoreInfo* pExternalSemaphoreInfo,
    VkExternalSemaphoreProperties*              pExternalSemaphoreProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_external_semaphore_capabilities
void vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalSemaphoreInfo* pExternalSemaphoreInfo,
    VkExternalSemaphoreProperties*              pExternalSemaphoreProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the semaphore capabilities.
- [`p_external_semaphore_info`] is a pointer to a [`PhysicalDeviceExternalSemaphoreInfo`] structure describing the parameters that would be consumed by [`create_semaphore`].
- [`p_external_semaphore_properties`] is a pointer to a [`ExternalSemaphoreProperties`] structure in which capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_external_semaphore_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceExternalSemaphoreInfo`] structure
-  [`p_external_semaphore_properties`] **must**  be a valid pointer to a [`ExternalSemaphoreProperties`] structure

# Related
- [`crate::vulkan1_1`]
- [`ExternalSemaphoreProperties`]
- [`PhysicalDevice`]
- [`PhysicalDeviceExternalSemaphoreInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        