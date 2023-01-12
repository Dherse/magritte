[VkDescriptorImageInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorImageInfo.html) - Structure specifying descriptor image information

# C Specifications
The [`DescriptorImageInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorImageInfo {
    VkSampler        sampler;
    VkImageView      imageView;
    VkImageLayout    imageLayout;
} VkDescriptorImageInfo;
```

# Members
- [`sampler`] is a sampler handle, and is used in descriptor updates for types `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` if the binding being updated does not use immutable samplers.
- [`image_view`] is [`crate::Handle::null`] or an image view handle, and is used in descriptor updates for types `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`.
- [`image_layout`] is the layout that the image subresources accessible from [`image_view`] will be in at the time this descriptor is accessed. [`image_layout`] is used in descriptor updates for types `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`.

# Description
Members of [`DescriptorImageInfo`] that are not used in an update (as
described above) are ignored.
## Valid Usage
-  [`image_view`] **must**  not be 2D or 2D array image view created from a 3D image
-    If [`image_view`] is created from a depth/stencil image, the `aspectMask` used to create the [`image_view`] **must**  include either `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT` but not both
-  [`image_layout`] **must**  match the actual [`ImageLayout`] of each subresource accessible from [`image_view`] at the time this descriptor is accessed as defined by the [image layout matching rules](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-layouts-matching-rule)
-    If [`sampler`] is used and the [`Format`] of the image is a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the image  **must**  have been created with `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, and the `aspectMask` of the [`image_view`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or (for three-plane formats only) `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::mutable_comparison_samplers`] is `VK_FALSE`, then [`sampler`] **must**  have been created with [`SamplerCreateInfo::compare_enable`] set to `VK_FALSE`

## Valid Usage (Implicit)
-    Both of [`image_view`], and [`sampler`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`ImageLayout`]
- [`ImageView`]
- [`Sampler`]
- [`WriteDescriptorSet`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        