[VkBufferOpaqueCaptureAddressCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html) - Request a specific address for a buffer

# C Specifications
To request a specific device address for a buffer, add a
[`BufferOpaqueCaptureAddressCreateInfo`] structure to the [`p_next`]
chain of the [`BufferCreateInfo`] structure.
The [`BufferOpaqueCaptureAddressCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkBufferOpaqueCaptureAddressCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint64_t           opaqueCaptureAddress;
} VkBufferOpaqueCaptureAddressCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_buffer_device_address
typedef VkBufferOpaqueCaptureAddressCreateInfo VkBufferOpaqueCaptureAddressCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`opaque_capture_address`] is the opaque capture address requested for the buffer.

# Description
If [`opaque_capture_address`] is zero, no specific address is requested.If [`opaque_capture_address`] is not zero, then it  **should**  be an address
retrieved from [`get_buffer_opaque_capture_address`] for an identically
created buffer on the same implementation.If this structure is not present, it is as if [`opaque_capture_address`] is
zero.Apps  **should**  avoid creating buffers with app-provided addresses and
implementation-provided addresses in the same process, to reduce the
likelihood of `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` errors.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`

# Related
- [`khr_buffer_device_address`]
- [`crate::vulkan1_2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        