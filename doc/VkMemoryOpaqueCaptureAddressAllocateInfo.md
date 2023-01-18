[VkMemoryOpaqueCaptureAddressAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html) - Request a specific address for a memory allocation

# C Specifications
To request a specific device address for a memory allocation, add a
[`MemoryOpaqueCaptureAddressAllocateInfo`] structure to the [`p_next`]
chain of the [`MemoryAllocateInfo`] structure.
The [`MemoryOpaqueCaptureAddressAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkMemoryOpaqueCaptureAddressAllocateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint64_t           opaqueCaptureAddress;
} VkMemoryOpaqueCaptureAddressAllocateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_buffer_device_address
typedef VkMemoryOpaqueCaptureAddressAllocateInfo VkMemoryOpaqueCaptureAddressAllocateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`opaque_capture_address`] is the opaque capture address requested for the memory allocation.

# Description
If [`opaque_capture_address`] is zero, no specific address is requested.If [`opaque_capture_address`] is not zero, it  **should**  be an address
retrieved from [`get_device_memory_opaque_capture_address`] on an identically
created memory allocation on the same implementation.If this structure is not present, it is as if [`opaque_capture_address`] is
zero.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`

# Related
- [`VK_KHR_buffer_device_address`]
- [`crate::vulkan1_2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        