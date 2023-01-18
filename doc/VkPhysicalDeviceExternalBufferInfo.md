[VkPhysicalDeviceExternalBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) - Structure specifying buffer creation parameters

# C Specifications
The [`PhysicalDeviceExternalBufferInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceExternalBufferInfo {
    VkStructureType                       sType;
    const void*                           pNext;
    VkBufferCreateFlags                   flags;
    VkBufferUsageFlags                    usage;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkPhysicalDeviceExternalBufferInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory_capabilities
typedef VkPhysicalDeviceExternalBufferInfo VkPhysicalDeviceExternalBufferInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`BufferCreateFlagBits`] describing additional parameters of the buffer, corresponding to [`BufferCreateInfo`]::[`flags`].
- [`usage`] is a bitmask of [`BufferUsageFlagBits`] describing the intended usage of the buffer, corresponding to [`BufferCreateInfo`]::[`usage`].
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the memory handle type that will be used with the memory associated with the buffer.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`BufferCreateFlagBits`] values
-  [`usage`] **must**  be a valid combination of [`BufferUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`crate::vulkan1_1`]
- [`BufferCreateFlags`]
- [`BufferUsageFlags`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]
- [`get_physical_device_external_buffer_properties`]
- [`get_physical_device_external_buffer_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        