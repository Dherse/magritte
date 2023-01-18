[VkImageCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html) - Bitmask specifying additional parameters of an image

# C Specifications
Bits which  **can**  be set in [`ImageCreateInfo::flags`], specifying
additional parameters of an image, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageCreateFlagBits {
    VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
    VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
    VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
    VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
    VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_ALIAS_BIT = 0x00000400,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT = 0x00000040,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT = 0x00000020,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 0x00000080,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_EXTENDED_USAGE_BIT = 0x00000100,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_PROTECTED_BIT = 0x00000800,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_CREATE_DISJOINT_BIT = 0x00000200,
  // Provided by VK_NV_corner_sampled_image
    VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV = 0x00002000,
  // Provided by VK_EXT_sample_locations
    VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 0x00001000,
  // Provided by VK_EXT_fragment_density_map
    VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT = 0x00004000,
  // Provided by VK_QCOM_fragment_density_map_offset
    VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM = 0x00008000,
  // Provided by VK_KHR_bind_memory2 with VK_KHR_device_group
    VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT,
  // Provided by VK_KHR_maintenance1
    VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR = VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT,
  // Provided by VK_KHR_maintenance2
    VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT,
  // Provided by VK_KHR_maintenance2
    VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR = VK_IMAGE_CREATE_EXTENDED_USAGE_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_IMAGE_CREATE_DISJOINT_BIT_KHR = VK_IMAGE_CREATE_DISJOINT_BIT,
  // Provided by VK_KHR_bind_memory2
    VK_IMAGE_CREATE_ALIAS_BIT_KHR = VK_IMAGE_CREATE_ALIAS_BIT,
} VkImageCreateFlagBits;
```

# Description
- [`SPARSE_BINDING`] specifies that the image will be backed using sparse memory binding.
- [`SPARSE_RESIDENCY`] specifies that the image  **can**  be partially backed using sparse memory binding. Images created with this flag  **must**  also be created with the [`SPARSE_BINDING`] flag.
- [`SPARSE_ALIASED`] specifies that the image will be backed using sparse memory binding with memory ranges that might also simultaneously be backing another image (or another portion of the same image). Images created with this flag  **must**  also be created with the [`SPARSE_BINDING`] flag.
- [`MUTABLE_FORMAT`] specifies that the image  **can**  be used to create a [`ImageView`] with a different format from the image. For [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) formats, [`MUTABLE_FORMAT`] specifies that a [`ImageView`] can be created of a *plane* of the image.
- [`CUBE_COMPATIBLE`] specifies that the image  **can**  be used to create a [`ImageView`] of type `VK_IMAGE_VIEW_TYPE_CUBE` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`.
- [`2D_ARRAY_COMPATIBLE`] specifies that the image  **can**  be used to create a [`ImageView`] of type `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`.
- [`PROTECTED`] specifies that the image is a protected image.
- [`SPLIT_INSTANCE_BIND_REGIONS`] specifies that the image  **can**  be used with a non-zero value of the `splitInstanceBindRegionCount` member of a [`BindImageMemoryDeviceGroupInfo`] structure passed into [`bind_image_memory2`]. This flag also has the effect of making the image use the standard sparse image block dimensions.
- [`BLOCK_TEXEL_VIEW_COMPATIBLE`] specifies that the image having a compressed format  **can**  be used to create a [`ImageView`] with an uncompressed format where each texel in the image view corresponds to a compressed texel block of the image.
- [`EXTENDED_USAGE`] specifies that the image  **can**  be created with usage flags that are not supported for the format the image is created with but are supported for at least one format a [`ImageView`] created from the image  **can**  have.
- [`DISJOINT`] specifies that an image with a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) **must**  have each plane separately bound to memory, rather than having a single memory binding for the whole image; the presence of this bit distinguishes a *disjoint image* from an image without this bit set.
- [`ALIAS`] specifies that two images created with     the same creation parameters and aliased to the same memory  **can**      interpret the contents of the memory consistently with each other,     subject to the rules described in the [Memory     Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section.     This flag further specifies that each plane of a *disjoint* image  **can**      share an in-memory non-linear representation with single-plane images,     and that a single-plane image  **can**  share an in-memory non-linear     representation with a plane of a multi-planar disjoint image, according     to the rules in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes).     If the `pNext` chain includes a [`ExternalMemoryImageCreateInfo`] or [`ExternalMemoryImageCreateInfoNV`]     structure whose `handleTypes` member is not `0`, it is as if     [`ALIAS`] is set.
- [`SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT`] specifies that an image with a depth or depth/stencil format  **can**  be used with custom sample locations when used as a depth/stencil attachment.
- [`CORNER_SAMPLED_NV`] specifies that the image is a [corner-sampled image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-images-corner-sampled).
- [`SUBSAMPLED_EXT`] specifies that an image  **can**  be in a subsampled format which  **may**  be more optimal when written as an attachment by a render pass that has a fragment density map attachment. Accessing a subsampled image has additional considerations:  - Image data read as an image sampler will have undefined values if the sampler was not created with `flags` containing `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT` or was not sampled through the use of a combined image sampler with an immutable sampler in [`DescriptorSetLayoutBinding`].  - Image data read with an input attachment will have undefined values if the contents were not written as an attachment in an earlier subpass of the same render pass.  - Image data read as an image sampler in the fragment shader will be additionally be read by the device during `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT` if [[`PhysicalDeviceFragmentDensityMap2PropertiesEXT::subsampled_coarse_reconstruction_early_access`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subsampledCoarseReconstructionEarlyAccess) is [`TRUE`] and the sampler was created with `flags` containing `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT`.  - Image data read with load operations are resampled to the fragment density of the render pass if [[`PhysicalDeviceFragmentDensityMap2PropertiesEXT::subsampled_loads`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subsampledLoads) is [`TRUE`]. Otherwise, values of image data are undefined.  - Image contents outside of the render area take on undefined values if the image is stored as a render pass attachment. 
- [`FRAGMENT_DENSITY_MAP_OFFSET_QCOM`] specifies that an image  **can**  be used in a render pass with non-zero [fragment density map offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets). In a renderpass with non-zero offsets, fragment density map attachments, input attachments, color attachments, depth/stencil attachment, resolve attachments, and preserve attachments  **must**  be created with VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM.
See [Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures) and
[Sparse Physical Device Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-physicalfeatures) for more
details.

# Related
- [`crate::vulkan1_0`]
- [`ImageCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        