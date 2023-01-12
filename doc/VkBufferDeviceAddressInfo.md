[VkBufferDeviceAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html) - Structure specifying the buffer to query an address for

# C Specifications
The [`BufferDeviceAddressInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkBufferDeviceAddressInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkBuffer           buffer;
} VkBufferDeviceAddressInfo;
```
or the equivalent
```c
// Provided by VK_KHR_buffer_device_address
typedef VkBufferDeviceAddressInfo VkBufferDeviceAddressInfoKHR;
```
or the equivalent
```c
// Provided by VK_EXT_buffer_device_address
typedef VkBufferDeviceAddressInfo VkBufferDeviceAddressInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`buffer`] specifies the buffer whose address is being queried.

# Description
## Valid Usage
-    If [`buffer`] is non-sparse and was not created with the `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT` flag, then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`buffer`] **must**  have been created with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`buffer`] **must**  be a valid [`Buffer`] handle

# Related
- [`crate::vulkan1_2`]
- [`Buffer`]
- [`StructureType`]
- [`get_buffer_device_address`]
- [`get_buffer_device_address_ext`]
- [`get_buffer_device_address_khr`]
- [`get_buffer_opaque_capture_address`]
- [`get_buffer_opaque_capture_address_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        