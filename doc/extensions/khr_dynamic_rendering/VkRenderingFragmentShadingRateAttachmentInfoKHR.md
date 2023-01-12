[VkRenderingFragmentShadingRateAttachmentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html) - Structure specifying fragment shading rate attachment information

# C Specifications
The [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure is
defined as:
```c
// Provided by VK_KHR_dynamic_rendering with VK_KHR_fragment_shading_rate
typedef struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkImageView        imageView;
    VkImageLayout      imageLayout;
    VkExtent2D         shadingRateAttachmentTexelSize;
} VkRenderingFragmentShadingRateAttachmentInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image_view`] is the image view that will be used as a fragment shading rate attachment.
- [`image_layout`] is the layout that [`image_view`] will be in during rendering.
- [`shading_rate_attachment_texel_size`] specifies the number of pixels corresponding to each texel in [`image_view`].

# Description
This structure can be included in the [`p_next`] chain of
[`RenderingInfo`] to define a
[fragment shading rate
attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment).
If [`image_view`] is [`crate::Handle::null`], or if this structure is not
specified, the implementation behaves as if a valid shading rate attachment
was specified with all texels specifying a single pixel per fragment.
## Valid Usage
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  be `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width` **must**  be a power of two value
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height` **must**  be a power of two value
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
-    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
-    If [`image_view`] is not [`crate::Handle::null`], the quotient of `shadingRateAttachmentTexelSize.width` and `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
-    If [`image_view`] is not [`crate::Handle::null`], the quotient of `shadingRateAttachmentTexelSize.height` and `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
-    If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value

# Related
- [`khr_dynamic_rendering`]
- [`khr_fragment_shading_rate`]
- [`Extent2D`]
- [`ImageLayout`]
- [`ImageView`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        