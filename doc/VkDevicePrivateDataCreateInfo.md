[VkDevicePrivateDataCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfo.html) - Reserve private data slots

# C Specifications
To reserve private data storage slots, add a
[`DevicePrivateDataCreateInfo`] structure to the [`p_next`] chain of
the [`DeviceCreateInfo`] structure.
Reserving slots in this manner is not strictly necessary, but doing so  **may** 
improve performance.
```c
// Provided by VK_VERSION_1_3
typedef struct VkDevicePrivateDataCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           privateDataSlotRequestCount;
} VkDevicePrivateDataCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_private_data
typedef VkDevicePrivateDataCreateInfo VkDevicePrivateDataCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`private_data_slot_request_count`] is the amount of slots to reserve.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`

# Related
- [`ext_private_data`]
- [`crate::vulkan1_3`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        