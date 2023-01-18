[VkMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) - Properties of External Memory Windows Handles

# C Specifications
The [`MemoryWin32HandlePropertiesKHR`] structure returned is defined as:
```c
// Provided by VK_KHR_external_memory_win32
typedef struct VkMemoryWin32HandlePropertiesKHR {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           memoryTypeBits;
} VkMemoryWin32HandlePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the specified windows handle  **can**  be imported as.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_external_memory_win32`]
- [`StructureType`]
- [`get_memory_win32_handle_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        