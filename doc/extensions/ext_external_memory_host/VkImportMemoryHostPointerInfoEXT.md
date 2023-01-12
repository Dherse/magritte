[VkImportMemoryHostPointerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) - Import memory from a host pointer

# C Specifications
To import memory from a host pointer, add a
[`ImportMemoryHostPointerInfoEXT`] structure to the [`p_next`] chain of
the [`MemoryAllocateInfo`] structure.
The [`ImportMemoryHostPointerInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_external_memory_host
typedef struct VkImportMemoryHostPointerInfoEXT {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalMemoryHandleTypeFlagBits    handleType;
    void*                                 pHostPointer;
} VkImportMemoryHostPointerInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type.
- [`host_pointer`] is the host pointer to import from.

# Description
Importing memory from a host pointer shares ownership of the memory between
the host and the Vulkan implementation.
The application  **can**  continue to access the memory through the host pointer
but it is the application’s responsibility to synchronize device and
non-device access to the payload as defined in
[Host Access to Device Memory Objects](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).Applications  **can**  import the same payload into multiple instances of Vulkan
and multiple times into a given Vulkan instance.
However, implementations  **may**  fail to import the same payload multiple times
into a given physical device due to platform constraints.Importing memory from a particular host pointer  **may**  not be possible due to
additional platform-specific restrictions beyond the scope of this
specification in which case the implementation  **must**  fail the memory import
operation with the error code `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`.Whether device memory objects imported from a host pointer hold a reference
to their payload is undefined.
As such, the application  **must**  ensure that the imported memory range remains
valid and accessible for the lifetime of the imported memory object.
## Valid Usage
-    If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported in [`ExternalMemoryProperties`]
-    If [`handle_type`] is not `0`, it  **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
-  [`host_pointer`] **must**  be a pointer aligned to an integer multiple of [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
-    If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`, [`host_pointer`] **must**  be a pointer to `allocationSize` number of bytes of host memory, where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure this structure is chained to
-    If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`, [`host_pointer`] **must**  be a pointer to `allocationSize` number of bytes of host mapped foreign memory, where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure this structure is chained to

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`ext_external_memory_host`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        