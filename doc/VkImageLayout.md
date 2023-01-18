[VkImageLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html) - Layout of image and image subresources

# C Specifications
The set of image layouts consists of:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageLayout {
    VK_IMAGE_LAYOUT_UNDEFINED = 0,
    VK_IMAGE_LAYOUT_GENERAL = 1,
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,
  // Provided by VK_VERSION_1_2
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL = 1000241000,
  // Provided by VK_VERSION_1_2
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL = 1000241001,
  // Provided by VK_VERSION_1_2
    VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL = 1000241002,
  // Provided by VK_VERSION_1_2
    VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL = 1000241003,
  // Provided by VK_VERSION_1_3
    VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL = 1000314000,
  // Provided by VK_VERSION_1_3
    VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL = 1000314001,
  // Provided by VK_KHR_swapchain
    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_LAYOUT_VIDEO_DECODE_DST_KHR = 1000024000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_LAYOUT_VIDEO_DECODE_SRC_KHR = 1000024001,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_LAYOUT_VIDEO_DECODE_DPB_KHR = 1000024002,
#endif
  // Provided by VK_KHR_shared_presentable_image
    VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR = 1000111000,
  // Provided by VK_EXT_fragment_density_map
    VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT = 1000218000,
  // Provided by VK_KHR_fragment_shading_rate
    VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR = 1000164003,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_LAYOUT_VIDEO_ENCODE_DST_KHR = 1000299000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_LAYOUT_VIDEO_ENCODE_SRC_KHR = 1000299001,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_LAYOUT_VIDEO_ENCODE_DPB_KHR = 1000299002,
#endif
  // Provided by VK_KHR_maintenance2
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
  // Provided by VK_KHR_maintenance2
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
  // Provided by VK_NV_shading_rate_image
    VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV = VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR,
  // Provided by VK_KHR_separate_depth_stencil_layouts
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
  // Provided by VK_KHR_separate_depth_stencil_layouts
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL,
  // Provided by VK_KHR_separate_depth_stencil_layouts
    VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
  // Provided by VK_KHR_separate_depth_stencil_layouts
    VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL,
  // Provided by VK_KHR_synchronization2
    VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR = VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL,
  // Provided by VK_KHR_synchronization2
    VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR = VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL,
} VkImageLayout;
```

# Description
The type(s) of device access supported by each layout are:
- [`UNDEFINED`] specifies that the layout is unknown. Image memory  **cannot**  be transitioned into this layout. This layout  **can**  be used as the `initialLayout` member of [`ImageCreateInfo`]. This layout  **can**  be used in place of the current image layout in a layout transition, but doing so will cause the contents of the image’s memory to be undefined.
- [`PREINITIALIZED`] specifies that an image’s memory is in a defined layout and  **can**  be populated by data, but that it has not yet been initialized by the driver. Image memory  **cannot**  be transitioned into this layout. This layout  **can**  be used as the `initialLayout` member of [`ImageCreateInfo`]. This layout is intended to be used as the initial layout for an image whose contents are written by the host, and hence the data  **can**  be written to memory immediately, without first executing a layout transition. Currently, [`PREINITIALIZED`] is only useful with [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource) images because there is not a standard layout defined for `VK_IMAGE_TILING_OPTIMAL` images.
- [`GENERAL`] supports all types of device access.
- [`ATTACHMENT_OPTIMAL`] specifies a layout that  **must**  only be used with attachment accesses in the graphics pipeline.
- [`READ_ONLY_OPTIMAL`] specifies a layout allowing read only access as an attachment, or in shaders as a sampled image, combined image/sampler, or input attachment.
- [`COLOR_ATTACHMENT_OPTIMAL`] **must**  only be used as a color or resolve attachment in a [`Framebuffer`]. This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` usage bit enabled.
- [`DEPTH_STENCIL_ATTACHMENT_OPTIMAL`] specifies a layout for both the depth and stencil aspects of a depth/stencil format image allowing read and write access as a depth/stencil attachment. It is equivalent to [`DEPTH_ATTACHMENT_OPTIMAL`] and [`STENCIL_ATTACHMENT_OPTIMAL`].
- [`DEPTH_STENCIL_READ_ONLY_OPTIMAL`] specifies a layout for both the depth and stencil aspects of a depth/stencil format image allowing read only access as a depth/stencil attachment or in shaders as a sampled image, combined image/sampler, or input attachment. It is equivalent to [`DEPTH_READ_ONLY_OPTIMAL`] and [`STENCIL_READ_ONLY_OPTIMAL`].
- [`DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`] specifies a layout for depth/stencil format images allowing read and write access to the stencil aspect as a stencil attachment, and read only access to the depth aspect as a depth attachment or in shaders as a sampled image, combined image/sampler, or input attachment. It is equivalent to [`DEPTH_READ_ONLY_OPTIMAL`] and [`STENCIL_ATTACHMENT_OPTIMAL`].
- [`DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`] specifies a layout for depth/stencil format images allowing read and write access to the depth aspect as a depth attachment, and read only access to the stencil aspect as a stencil attachment or in shaders as a sampled image, combined image/sampler, or input attachment. It is equivalent to [`DEPTH_ATTACHMENT_OPTIMAL`] and [`STENCIL_READ_ONLY_OPTIMAL`].
- [`DEPTH_ATTACHMENT_OPTIMAL`] specifies a layout for the depth aspect of a depth/stencil format image allowing read and write access as a depth attachment.
- [`DEPTH_READ_ONLY_OPTIMAL`] specifies a layout for the depth aspect of a depth/stencil format image allowing read-only access as a depth attachment or in shaders as a sampled image, combined image/sampler, or input attachment.
- [`STENCIL_ATTACHMENT_OPTIMAL`] specifies a layout for the stencil aspect of a depth/stencil format image allowing read and write access as a stencil attachment.
- [`STENCIL_READ_ONLY_OPTIMAL`] specifies a layout for the stencil aspect of a depth/stencil format image allowing read-only access as a stencil attachment or in shaders as a sampled image, combined image/sampler, or input attachment.
- [`SHADER_READ_ONLY_OPTIMAL`] specifies a layout allowing read-only access in a shader as a sampled image, combined image/sampler, or input attachment. This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_SAMPLED_BIT` or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` usage bits enabled.
- [`TRANSFER_SRC_OPTIMAL`] **must**  only be used as a source image of a transfer command (see the definition of [`VK_PIPELINE_STAGE_TRANSFER_BIT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-transfer)). This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage bit enabled.
- [`TRANSFER_DST_OPTIMAL`] **must**  only be used as a destination image of a transfer command. This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage bit enabled.
- [`PRESENT_SRC_KHR`] **must**  only be used for presenting a presentable image for display. A swapchain’s image  **must**  be transitioned to this layout before calling [`queue_present_khr`], and  **must**  be transitioned away from this layout after calling [`acquire_next_image_khr`].
- [`SHARED_PRESENT_KHR`] is valid only for shared presentable images, and  **must**  be used for any usage the image supports.
- [`FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`] **must**      only be used as a     [fragment shading rate     attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) or     [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image).     This layout is valid only for image subresources of images created with     the `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` usage     bit enabled.
- [`FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`] **must**  only be used as a fragment density map attachment in a [`RenderPass`]. This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT` usage bit enabled.
- [`VIDEO_DECODE_DST_KHR`] **must**  only be used as a decode output image of a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations). This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` usage bit enabled.
- [`VIDEO_DECODE_SRC_KHR`] is reserved for future use.
- [`VIDEO_DECODE_DPB_KHR`] **must**  only be used as a decode source or destination image of a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations). This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` usage bit enabled.
- [`VIDEO_ENCODE_DST_KHR`] is reserved for future use.
- [`VIDEO_ENCODE_SRC_KHR`] **must**  only be used as a encode source image of a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations). This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR` usage bit enabled.
- [`VIDEO_ENCODE_DPB_KHR`] **must**  only be used as a encode source or destination image of a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations). This layout is valid only for image subresources of images created with the `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR` usage bit enabled.
The layout of each image subresource is not a state of the image subresource
itself, but is rather a property of how the data in memory is organized, and
thus for each mechanism of accessing an image in the API the application
 **must**  specify a parameter or structure member that indicates which image
layout the image subresource(s) are considered to be in when the image will
be accessed.
For transfer commands, this is a parameter to the command (see [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears)
and [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies)).
For use as a framebuffer attachment, this is a member in the substructures
of the [`RenderPassCreateInfo`] (see [Render Pass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass)).
For use in a descriptor set, this is a member in the
[`DescriptorImageInfo`] structure (see [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates)).

# Related
- [`crate::vulkan1_0`]
- [`AttachmentDescription`]
- [`AttachmentDescription2`]
- [`AttachmentDescriptionStencilLayout`]
- [`AttachmentReference`]
- [`AttachmentReference2`]
- [`AttachmentReferenceStencilLayout`]
- [`BlitImageInfo2`]
- [`CopyBufferToImageInfo2`]
- [`CopyImageInfo2`]
- [`CopyImageToBufferInfo2`]
- [`DescriptorImageInfo`]
- [`ImageCreateInfo`]
- [`ImageMemoryBarrier`]
- [`ImageMemoryBarrier2`]
- [`RenderingAttachmentInfo`]
- [`RenderingFragmentDensityMapAttachmentInfoEXT`]
- [`RenderingFragmentShadingRateAttachmentInfoKHR`]
- [`ResolveImageInfo2`]
- [`cmd_bind_invocation_mask_huawei`]
- [`cmd_bind_shading_rate_image_nv`]
- [`cmd_blit_image`]
- [`cmd_clear_color_image`]
- [`cmd_clear_depth_stencil_image`]
- [`cmd_copy_buffer_to_image`]
- [`cmd_copy_image`]
- [`cmd_copy_image_to_buffer`]
- [`cmd_resolve_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        