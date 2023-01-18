[VkCuFunctionCreateInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionCreateInfoNVX.html) - Stub description of VkCuFunctionCreateInfoNVX

# C Specifications
There is currently no specification language written for this type.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_NVX_binary_import
typedef struct VkCuFunctionCreateInfoNVX {
    VkStructureType    sType;
    const void*        pNext;
    VkCuModuleNVX      module;
    const char*        pName;
} VkCuFunctionCreateInfoNVX;
```

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX`
-  [`p_next`] **must**  be `NULL`
-  [`module`] **must**  be a valid [`CuModuleNVX`] handle
-  [`name`] **must**  be a null-terminated UTF-8 string

# Related
- [`VK_NVX_binary_import`]
- [`CuModuleNVX`]
- [`StructureType`]
- [`create_cu_function_nvx`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        