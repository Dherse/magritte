[VkImageView](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html) - Opaque handle to an image view object

# C Specifications
Image objects are not directly accessed by pipeline shaders for reading or
writing image data.
Instead, *image views* representing contiguous ranges of the image
subresources and containing additional metadata are used for that purpose.
Views  **must**  be created on images of compatible types, and  **must**  represent a
valid subset of image subresources.Image views are represented by [`ImageView`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImageView)
```

# Related
- [`crate::vulkan1_0`]
- [`DescriptorImageInfo`]
- [`FramebufferCreateInfo`]
- [`ImageViewHandleInfoNVX`]
- [`RenderPassAttachmentBeginInfo`]
- [`RenderingAttachmentInfo`]
- [`RenderingFragmentDensityMapAttachmentInfoEXT`]
- [`RenderingFragmentShadingRateAttachmentInfoKHR`]
- [`VideoPictureResourceKHR`]
- [`cmd_bind_invocation_mask_huawei`]
- [`cmd_bind_shading_rate_image_nv`]
- [`create_image_view`]
- [`destroy_image_view`]
- [`get_image_view_address_nvx`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        