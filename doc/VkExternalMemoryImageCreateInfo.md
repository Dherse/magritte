[VkExternalMemoryImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html) - Specify that an image may be backed by external memory

# C Specifications
To define a set of external memory handle types that  **may**  be used as backing
store for an image, add a [`ExternalMemoryImageCreateInfo`] structure to
the [`p_next`] chain of the [`ImageCreateInfo`] structure.
The [`ExternalMemoryImageCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkExternalMemoryImageCreateInfo {
    VkStructureType                    sType;
    const void*                        pNext;
    VkExternalMemoryHandleTypeFlags    handleTypes;
} VkExternalMemoryImageCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_external_memory
typedef VkExternalMemoryImageCreateInfo VkExternalMemoryImageCreateInfoKHR;
```

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying one or more external memory handle types.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`
-  [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [VkExternalMemoryHandleTypeFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        