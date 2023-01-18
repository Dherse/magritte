[VkExportFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html) - Structure specifying handle types that can be exported from a fence

# C Specifications
To create a fence whose payload  **can**  be exported to external handles, add a
[`ExportFenceCreateInfo`] structure to the [`p_next`] chain of the
[`FenceCreateInfo`] structure.
The [`ExportFenceCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExportFenceCreateInfo {
    VkStructureType                   sType;
    const void*                       pNext;
    VkExternalFenceHandleTypeFlags    handleTypes;
} VkExportFenceCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_fence
typedef VkExportFenceCreateInfo VkExportFenceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] specifying one or more fence handle types the application  **can**  export from the resulting fence. The application  **can**  request multiple handle types for the same fence.

# Description
## Valid Usage
-    The bits in [`handle_types`] **must**  be supported and compatible, as reported by [`ExternalFenceProperties`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`
-  [`handle_types`] **must**  be a valid combination of [`ExternalFenceHandleTypeFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [`ExternalFenceHandleTypeFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        