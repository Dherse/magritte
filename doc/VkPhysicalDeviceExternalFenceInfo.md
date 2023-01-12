[VkPhysicalDeviceExternalFenceInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) - Structure specifying fence creation parameters.

# C Specifications
The [`PhysicalDeviceExternalFenceInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceExternalFenceInfo {
    VkStructureType                      sType;
    const void*                          pNext;
    VkExternalFenceHandleTypeFlagBits    handleType;
} VkPhysicalDeviceExternalFenceInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence_capabilities
typedef VkPhysicalDeviceExternalFenceInfo VkPhysicalDeviceExternalFenceInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying an external fence handle type for which capabilities will be returned.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceHandleTypeFlagBits`]
- [`StructureType`]
- [`get_physical_device_external_fence_properties`]
- [`get_physical_device_external_fence_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        