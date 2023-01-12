[vkGetPhysicalDeviceExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html) - Query external handle types supported by buffers

# C Specifications
To query the external handle types supported by buffers, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceExternalBufferProperties(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalBufferInfo*   pExternalBufferInfo,
    VkExternalBufferProperties*                 pExternalBufferProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_external_memory_capabilities
void vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceExternalBufferInfo*   pExternalBufferInfo,
    VkExternalBufferProperties*                 pExternalBufferProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the buffer capabilities.
- [`p_external_buffer_info`] is a pointer to a [`PhysicalDeviceExternalBufferInfo`] structure describing the parameters that would be consumed by [`create_buffer`].
- [`p_external_buffer_properties`] is a pointer to a [`ExternalBufferProperties`] structure in which capabilities are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_external_buffer_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceExternalBufferInfo`] structure
-  [`p_external_buffer_properties`] **must**  be a valid pointer to a [`ExternalBufferProperties`] structure

# Related
- [`crate::vulkan1_1`]
- [`ExternalBufferProperties`]
- [`PhysicalDevice`]
- [`PhysicalDeviceExternalBufferInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        