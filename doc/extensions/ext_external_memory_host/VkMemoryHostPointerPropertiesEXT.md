[VkMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) - Properties of external memory host pointer

# C Specifications
The [`MemoryHostPointerPropertiesEXT`] structure is defined as:
```c
// Provided by VK_EXT_external_memory_host
typedef struct VkMemoryHostPointerPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           memoryTypeBits;
} VkMemoryHostPointerPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the specified host pointer  **can**  be imported as.

# Description
The value returned by [`memory_type_bits`] **must**  only include bits that
identify memory types which are host visible.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`
-  [`p_next`] **must**  be `NULL`

# Related
- [`ext_external_memory_host`]
- [`StructureType`]
- [`get_memory_host_pointer_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        