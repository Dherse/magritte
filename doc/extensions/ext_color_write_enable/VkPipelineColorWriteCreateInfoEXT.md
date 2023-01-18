[VkPipelineColorWriteCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) - Structure specifying color write state of a newly created pipeline

# C Specifications
The [`PipelineColorWriteCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_color_write_enable
typedef struct VkPipelineColorWriteCreateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           attachmentCount;
    const VkBool32*    pColorWriteEnables;
} VkPipelineColorWriteCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attachment_count`] is the number of [`Bool32`] elements in [`color_write_enables`].
- [`color_write_enables`] is a pointer to an array of per target attachment boolean values specifying whether color writes are enabled for the given attachment.

# Description
When this structure is included in the [`p_next`] chain of
[`PipelineColorBlendStateCreateInfo`], it defines per-attachment color
write state.
If this structure is not included in the [`p_next`] chain, it is equivalent
to specifying this structure with [`attachment_count`] equal to the
[`attachment_count`] member of [`PipelineColorBlendStateCreateInfo`],
and [`color_write_enables`] pointing to an array of as many [`TRUE`]
values.If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable) feature is not enabled
on the device, all [`Bool32`] elements in the
[`color_write_enables`] array  **must**  be [`TRUE`].Color Write Enable interacts with the [Color
Write Mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-mask) as follows:
- If `colorWriteEnable` is [`TRUE`], writes to the attachment are determined by the `colorWriteMask`.
- If `colorWriteEnable` is [`FALSE`], the `colorWriteMask` is ignored and writes to all components of the attachment are disabled. This is equivalent to specifying a `colorWriteMask` of 0.

## Valid Usage
-    If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable) feature is not enabled, all elements of [`color_write_enables`] **must**  be [`TRUE`]
-  [`attachment_count`] **must**  be equal to the [`attachment_count`] member of the [`PipelineColorBlendStateCreateInfo`] structure specified during pipeline creation
-  [`attachment_count`] **must**  be less than or equal to the `maxColorAttachments` member of [`PhysicalDeviceLimits`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
-    If [`attachment_count`] is not `0`, [`color_write_enables`] **must**  be a valid pointer to an array of [`attachment_count`][`Bool32`] values

# Related
- [`VK_EXT_color_write_enable`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        