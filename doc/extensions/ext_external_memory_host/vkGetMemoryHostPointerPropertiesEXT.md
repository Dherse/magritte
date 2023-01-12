[vkGetMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) - Get properties of external memory host pointer

# C Specifications
To determine the correct parameters to use when importing host pointers,
call:
```c
// Provided by VK_EXT_external_memory_host
VkResult vkGetMemoryHostPointerPropertiesEXT(
    VkDevice                                    device,
    VkExternalMemoryHandleTypeFlagBits          handleType,
    const void*                                 pHostPointer,
    VkMemoryHostPointerPropertiesEXT*           pMemoryHostPointerProperties);
```

# Parameters
- [`device`] is the logical device that will be importing [`p_host_pointer`].
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the handle [`p_host_pointer`].
- [`p_host_pointer`] is the host pointer to import from.
- [`p_memory_host_pointer_properties`] is a pointer to a [`MemoryHostPointerPropertiesEXT`] structure in which the host pointer properties are returned.

# Description
## Valid Usage
-  [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
-  [`p_host_pointer`] **must**  be a pointer aligned to an integer multiple of [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
-    If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`, [`p_host_pointer`] **must**  be a pointer to host memory
-    If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`, [`p_host_pointer`] **must**  be a pointer to host mapped foreign memory

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
-  [`p_memory_host_pointer_properties`] **must**  be a valid pointer to a [`MemoryHostPointerPropertiesEXT`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`ext_external_memory_host`]
- [`Device`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`MemoryHostPointerPropertiesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        