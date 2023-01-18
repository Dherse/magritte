[VkSysmemColorSpaceFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSysmemColorSpaceFUCHSIA.html) - Structure describing the buffer collections color space

# C Specifications
The [`SysmemColorSpaceFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkSysmemColorSpaceFUCHSIA {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           colorSpace;
} VkSysmemColorSpaceFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`color_space`] value of the Sysmem `ColorSpaceType`

# Description
## Valid Usage
-  [`color_space`] **must**  be a `ColorSpaceType` as defined in `fuchsia.sysmem/image_formats.fidl`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`BufferCollectionPropertiesFUCHSIA`]
- [`ImageFormatConstraintsInfoFUCHSIA`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        