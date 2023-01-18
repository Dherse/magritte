[VkBufferDeviceAddressCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) - Request a specific address for a buffer

# C Specifications
Alternatively, to
request a specific device address for a buffer, add a
[`BufferDeviceAddressCreateInfoEXT`] structure to the [`p_next`] chain
of the [`BufferCreateInfo`] structure.
The [`BufferDeviceAddressCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_buffer_device_address
typedef struct VkBufferDeviceAddressCreateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceAddress    deviceAddress;
} VkBufferDeviceAddressCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`device_address`] is the device address requested for the buffer.

# Description
If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not zero, then it  **must**  be an address retrieved
from an identically created buffer on the same implementation.
The buffer  **must**  also be bound to an identically created
[`DeviceMemory`] object.If this structure is not present, it is as if [`device_address`] is zero.Apps  **should**  avoid creating buffers with app-provided addresses and
implementation-provided addresses in the same process, to reduce the
likelihood of `VK_ERROR_INVALID_DEVICE_ADDRESS_EXT` errors.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`

# Related
- [`VK_EXT_buffer_device_address`]
- [`DeviceAddress`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        