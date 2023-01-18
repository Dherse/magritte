[VkImportFenceFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html) - (None)

# C Specifications
The [`ImportFenceFdInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_fence_fd
typedef struct VkImportFenceFdInfoKHR {
    VkStructureType                      sType;
    const void*                          pNext;
    VkFence                              fence;
    VkFenceImportFlags                   flags;
    VkExternalFenceHandleTypeFlagBits    handleType;
    int                                  fd;
} VkImportFenceFdInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fence`] is the fence into which the payload will be imported.
- [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the fence payload import operation.
- [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of [`fd`].
- [`fd`] is the external handle to import.

# Description
The handle types supported by [`handle_type`] are:
## Valid Usage
-  [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportFenceFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-fd) table
-  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
If [`handle_type`] is `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT`, the
special value `-1` for [`fd`] is treated like a valid sync file descriptor
referring to an object that has already signaled.
The import operation will succeed and the [`Fence`] will have a
temporarily imported payload as if a valid file descriptor had been
provided.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`fence`] **must**  be a valid [`Fence`] handle
-  [`flags`] **must**  be a valid combination of [`FenceImportFlagBits`] values
-  [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value

## Host Synchronization
- Host access to [`fence`] **must**  be externally synchronized

# Related
- [`VK_KHR_external_fence_fd`]
- [`ExternalFenceHandleTypeFlagBits`]
- [`Fence`]
- [`FenceImportFlags`]
- [`StructureType`]
- [`import_fence_fd_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        