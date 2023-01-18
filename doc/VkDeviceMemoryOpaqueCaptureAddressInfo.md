[VkDeviceMemoryOpaqueCaptureAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html) - Structure specifying the memory object to query an address for

# C Specifications
The [`DeviceMemoryOpaqueCaptureAddressInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkDeviceMemoryOpaqueCaptureAddressInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceMemory     memory;
} VkDeviceMemoryOpaqueCaptureAddressInfo;
```
or the equivalent
```c
// Provided by VK_KHR_buffer_device_address
typedef VkDeviceMemoryOpaqueCaptureAddressInfo VkDeviceMemoryOpaqueCaptureAddressInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] specifies the memory whose address is being queried.

# Description
## Valid Usage
-  [`memory`] **must**  have been allocated with `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle

# Related
- [`VK_KHR_buffer_device_address`]
- [`crate::vulkan1_2`]
- [`DeviceMemory`]
- [`StructureType`]
- [`get_device_memory_opaque_capture_address`]
- [`get_device_memory_opaque_capture_address_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        