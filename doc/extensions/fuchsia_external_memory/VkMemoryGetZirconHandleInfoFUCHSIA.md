[VkMemoryGetZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html) - Structure specifying export parameters for Zircon handle to device memory

# C Specifications
[`MemoryGetZirconHandleInfoFUCHSIA`] is defined as:
```c
// Provided by VK_FUCHSIA_external_memory
typedef struct VkMemoryGetZirconHandleInfoFUCHSIA {
    VkStructureType                       sType;
    const void*                           pNext;
    VkDeviceMemory                        memory;
    VkExternalMemoryHandleTypeFlagBits    handleType;
} VkMemoryGetZirconHandleInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] the [`DeviceMemory`] being exported.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the handle pointed to by [`get_memory_zircon_handle_fuchsia`]`::pZirconHandle`.

# Description
## Valid Usage
-  [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
-  [`handle_type`] **must**  have been included in the `handleTypes` field of the [`ExportMemoryAllocateInfo`] structure when the external memory was allocated

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`fuchsia_external_memory`]
- [`DeviceMemory`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]
- [`get_memory_zircon_handle_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        