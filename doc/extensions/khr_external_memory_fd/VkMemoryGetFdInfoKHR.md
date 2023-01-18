[VkMemoryGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html) - Structure describing a POSIX FD semaphore export operation

# C Specifications
The [`MemoryGetFdInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_memory_fd
typedef struct VkMemoryGetFdInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkDeviceMemory                        memory;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkMemoryGetFdInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] is the memory object from which the handle will be exported.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of handle requested.

# Description
The properties of the file descriptor exported depend on the value of
[`handle_type`].
See [`ExternalMemoryHandleTypeFlagBits`] for a description of the
properties of the defined external memory handle types.
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created
-  [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT` or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`VK_KHR_external_memory_fd`]
- [`DeviceMemory`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]
- [`get_memory_fd_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        