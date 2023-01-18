[VkImageViewHandleInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewHandleInfoNVX.html) - Structure specifying the image view for handle queries

# C Specifications
The [`ImageViewHandleInfoNVX`] structure is defined as:
```c
// Provided by VK_NVX_image_view_handle
typedef struct VkImageViewHandleInfoNVX {
    VkStructureType     sType;
    const void*         pNext;
    VkImageView         imageView;
    VkDescriptorType    descriptorType;
    VkSampler           sampler;
} VkImageViewHandleInfoNVX;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image_view`] is the image view to query.
- [`descriptor_type`] is the type of descriptor for which to query a handle.
- [`sampler`] is the sampler to combine with the image view when generating the handle.

# Description
## Valid Usage
-  [`descriptor_type`] **must**  be `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
-  [`sampler`] **must**  be a valid [`Sampler`] if [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
-    If descriptorType is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, the image that [`image_view`] was created from  **must**  have been created with the `VK_IMAGE_USAGE_SAMPLED_BIT` usage bit set
-    If descriptorType is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, the image that [`image_view`] was created from  **must**  have been created with the `VK_IMAGE_USAGE_STORAGE_BIT` usage bit set

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX`
-  [`p_next`] **must**  be `NULL`
-  [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`descriptor_type`] **must**  be a valid [`DescriptorType`] value
-    If [`sampler`] is not [`crate::Handle::null`], [`sampler`] **must**  be a valid [`Sampler`] handle
-    Both of [`image_view`], and [`sampler`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_NVX_image_view_handle`]
- [`DescriptorType`]
- [`ImageView`]
- [`Sampler`]
- [`StructureType`]
- [`get_image_view_handle_nvx`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        