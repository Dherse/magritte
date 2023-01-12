[VkFenceGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle fence export operation

# C Specifications
The [`FenceGetWin32HandleInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_fence_win32
typedef struct VkFenceGetWin32HandleInfoKHR {
    VkStructureType                      sType;
    const void*                          pNext;
    VkFence                              fence;
    VkExternalFenceHandleTypeFlagBits    handleType;
} VkFenceGetWin32HandleInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fence`] is the fence from which state will be exported.
- [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle requested.

# Description
The properties of the handle returned depend on the value of
[`handle_type`].
See [`ExternalFenceHandleTypeFlagBits`] for a description of the
properties of the defined external fence handle types.
## Valid Usage
-  [`handle_type`] **must**  have been included in [`ExportFenceCreateInfo::handle_types`] when the [`fence`]’s current payload was created
-    If [`handle_type`] is defined as an NT handle, [`get_fence_win32_handle_khr`] **must**  be called no more than once for each valid unique combination of [`fence`] and [`handle_type`]
-  [`fence`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
-    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`] **must**  be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling) pending execution
-  [`handle_type`] **must**  be defined as an NT handle or a global share handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`fence`] **must**  be a valid [`Fence`] handle
-  [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value

# Related
- [`khr_external_fence_win32`]
- [`ExternalFenceHandleTypeFlagBits`]
- [`Fence`]
- [`StructureType`]
- [`get_fence_win32_handle_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        