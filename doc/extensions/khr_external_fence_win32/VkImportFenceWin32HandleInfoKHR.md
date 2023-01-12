[VkImportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) - (None)

# C Specifications
The [`ImportFenceWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_fence_win32
typedef struct VkImportFenceWin32HandleInfoKHR {
    VkStructureType                      sType;
    const void*                          pNext;
    VkFence                              fence;
    VkFenceImportFlags                   flags;
    VkExternalFenceHandleTypeFlagBits    handleType;
    HANDLE                               handle;
    LPCWSTR                              name;
} VkImportFenceWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fence`] is the fence into which the state will be imported.
- [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the fence payload import operation.
- [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of [`handle`].
- [`handle`] is `NULL` or the external handle to import.
- [`name`] is `NULL` or a null-terminated UTF-16 string naming the underlying synchronization primitive to import.

# Description
The handle types supported by [`handle_type`] are:
## Valid Usage
-  [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportFenceWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-win32) table
-    If [`handle_type`] is not `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, [`name`] **must**  be `NULL`
-    If [`handle`] is `NULL`, [`name`] **must**  name a valid synchronization primitive of the type specified by [`handle_type`]
-    If [`name`] is `NULL`, [`handle`] **must**  be a valid handle of the type specified by [`handle_type`]
-    If [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
-    If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
-    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`fence`] **must**  be a valid [`Fence`] handle
-  [`flags`] **must**  be a valid combination of [`FenceImportFlagBits`] values

## Host Synchronization
- Host access to [`fence`] **must**  be externally synchronized

# Related
- [`khr_external_fence_win32`]
- [`ExternalFenceHandleTypeFlagBits`]
- [`Fence`]
- [VkFenceImportFlags]()
- [`StructureType`]
- [`import_fence_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        