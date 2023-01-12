[VkExportSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html) - Structure specifying handle types that can be exported from a semaphore

# C Specifications
To create a semaphore whose payload  **can**  be exported to external handles,
add a [`ExportSemaphoreCreateInfo`] structure to the [`p_next`] chain
of the [`SemaphoreCreateInfo`] structure.
The [`ExportSemaphoreCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExportSemaphoreCreateInfo {
    VkStructureType                       sType;
    const void*                           pNext;
    VkExternalSemaphoreHandleTypeFlags    handleTypes;
} VkExportSemaphoreCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_semaphore
typedef VkExportSemaphoreCreateInfo VkExportSemaphoreCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying one or more semaphore handle types the application  **can**  export from the resulting semaphore. The application  **can**  request multiple handle types for the same semaphore.

# Description
## Valid Usage
-    The bits in [`handle_types`] **must**  be supported and compatible, as reported by [`ExternalSemaphoreProperties`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`
-  [`handle_types`] **must**  be a valid combination of [`ExternalSemaphoreHandleTypeFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [VkExternalSemaphoreHandleTypeFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        