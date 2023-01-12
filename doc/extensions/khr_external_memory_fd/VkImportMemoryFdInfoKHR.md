[VkImportMemoryFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html) - Import memory created on the same physical device from a file descriptor

# C Specifications
To import memory from a POSIX file descriptor handle, add a
[`ImportMemoryFdInfoKHR`] structure to the [`p_next`] chain of the
[`MemoryAllocateInfo`] structure.
The [`ImportMemoryFdInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_memory_fd
typedef struct VkImportMemoryFdInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalMemoryHandleTypeFlagBits    handleType;
    int                                   fd;
} VkImportMemoryFdInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type of [`fd`].
- [`fd`] is the external handle to import.

# Description
Importing memory from a file descriptor transfers ownership of the file
descriptor from the application to the Vulkan implementation.
The application  **must**  not perform any operations on the file descriptor
after a successful import.
The imported memory object holds a reference to its payload.Applications  **can**  import the same payload into multiple instances of Vulkan,
into the same instance from which it was exported, and multiple times into a
given Vulkan instance.
In all cases, each import operation  **must**  create a distinct
[`DeviceMemory`] object.
## Valid Usage
-    If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported by [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
-    The memory from which [`fd`] was exported  **must**  have been created on the same underlying physical device as `device`
-    If [`handle_type`] is not `0`, it  **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT` or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
-    If [`handle_type`] is not `0`, [`fd`] **must**  be a valid handle of the type specified by [`handle_type`]
-    The memory represented by [`fd`] **must**  have been created from a physical device and driver that is compatible with `device` and [`handle_type`], as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
-  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
-    If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value

# Related
- [`khr_external_memory_fd`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        