[VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) - Structure describing advanced blending limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_blend_operation_advanced
typedef struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           advancedBlendMaxColorAttachments;
    VkBool32           advancedBlendIndependentBlend;
    VkBool32           advancedBlendNonPremultipliedSrcColor;
    VkBool32           advancedBlendNonPremultipliedDstColor;
    VkBool32           advancedBlendCorrelatedOverlap;
    VkBool32           advancedBlendAllOperations;
} VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`advanced_blend_max_color_attachments`] is one greater than the highest color attachment index that  **can**  be used in a subpass, for a pipeline that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
- [`advanced_blend_independent_blend`] specifies whether advanced blend operations  **can**  vary per-attachment.
- [`advanced_blend_non_premultiplied_src_color`] specifies whether the source color  **can**  be treated as non-premultiplied. If this is `VK_FALSE`, then [`PipelineColorBlendAdvancedStateCreateInfoEXT::src_premultiplied`] **must**  be `VK_TRUE`.
- [`advanced_blend_non_premultiplied_dst_color`] specifies whether the destination color  **can**  be treated as non-premultiplied. If this is `VK_FALSE`, then [`PipelineColorBlendAdvancedStateCreateInfoEXT::dst_premultiplied`] **must**  be `VK_TRUE`.
- [`advanced_blend_correlated_overlap`] specifies whether the overlap mode  **can**  be treated as correlated. If this is `VK_FALSE`, then [`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`] **must**  be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
- [`advanced_blend_all_operations`] specifies whether all advanced blend operation enums are supported. See the valid usage of [`PipelineColorBlendAttachmentState`].

# Description
If the [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`

# Related
- [`ext_blend_operation_advanced`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        