[VkMemoryGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle semaphore export operation

# C Specifications
The [`MemoryGetWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_memory_win32
typedef struct VkMemoryGetWin32HandleInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkDeviceMemory                        memory;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkMemoryGetWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] is the memory object from which the handle will be exported.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of handle requested.

# Description
The properties of the handle returned depend on the value of
[`handle_type`].
See [`ExternalMemoryHandleTypeFlagBits`] for a description of the
properties of the defined external memory handle types.
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`] when [`memory`] was created
-    If [`handle_type`] is defined as an NT handle, [`get_memory_win32_handle_khr`] **must**  be called no more than once for each valid unique combination of [`memory`] and [`handle_type`]
-  [`handle_type`] **must**  be defined as an NT handle or a global share handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`khr_external_memory_win32`]
- [`DeviceMemory`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]
- [`get_memory_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        