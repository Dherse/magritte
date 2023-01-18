[VkMemoryGetRemoteAddressInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html) - Structure describing a remote accessible address export operation

# C Specifications
The [`MemoryGetRemoteAddressInfoNV`] structure is defined as:
```c
// Provided by VK_NV_external_memory_rdma
typedef struct VkMemoryGetRemoteAddressInfoNV {
    VkStructureType                       sType;
    const void*                           pNext;
    VkDeviceMemory                        memory;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkMemoryGetRemoteAddressInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] is the memory object from which the remote accessible address will be exported.
- [`handle_type`] is the type of handle requested.

# Description
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`VK_NV_external_memory_rdma`]
- [`DeviceMemory`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]
- [`get_memory_remote_address_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        