[VkImageConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFUCHSIA.html) - Structure of image-based buffer collection constraints

# C Specifications
The [`ImageConstraintsInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkImageConstraintsInfoFUCHSIA {
    VkStructureType                               sType;
    const void*                                   pNext;
    uint32_t                                      formatConstraintsCount;
    const VkImageFormatConstraintsInfoFUCHSIA*    pFormatConstraints;
    VkBufferCollectionConstraintsInfoFUCHSIA      bufferCollectionConstraints;
    VkImageConstraintsInfoFlagsFUCHSIA            flags;
} VkImageConstraintsInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format_constraints_count`] is the number of elements in [`format_constraints`].
- [`format_constraints`] is a pointer to an array of [`ImageFormatConstraintsInfoFUCHSIA`] structures of size [`format_constraints_count`] that is used to further constrain buffer collection format selection for image-based buffer collections.
- [`buffer_collection_constraints`] is a [`BufferCollectionConstraintsInfoFUCHSIA`] structure used to supply parameters for the negotiation and allocation for buffer-based buffer collections.
- [`flags`] is a [`ImageConstraintsInfoFlagBitsFUCHSIA`] value specifying hints about the type of memory Sysmem should allocate for the buffer collection.

# Description
## Valid Usage
-    All elements of [`format_constraints`] **must**  have at least one bit set in its [`ImageFormatConstraintsInfoFUCHSIA::required_format_features`]
-    If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_SAMPLED_BIT`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
-    If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_STORAGE_BIT`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT`
-    If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain at least one of `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) is enabled, and [`format_constraints`]`::imageCreateInfo`::`usage` contains `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, then [`format_constraints`]`::requiredFormatFeatures` **must**  contain `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`format_constraints`] **must**  be a valid pointer to an array of [`format_constraints_count`] valid [`ImageFormatConstraintsInfoFUCHSIA`] structures
-  [`buffer_collection_constraints`] **must**  be a valid [`BufferCollectionConstraintsInfoFUCHSIA`] structure
-  [`flags`] **must**  be a valid combination of [`ImageConstraintsInfoFlagBitsFUCHSIA`] values
-  [`format_constraints_count`] **must**  be greater than `0`

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`BufferCollectionConstraintsInfoFUCHSIA`]
- [`ImageConstraintsInfoFlagsFUCHSIA`]
- [`ImageFormatConstraintsInfoFUCHSIA`]
- [`StructureType`]
- [`set_buffer_collection_image_constraints_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        