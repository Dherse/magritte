[VkImageUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html) - Bitmask specifying intended usage of an image

# C Specifications
Bits which  **can**  be set in
  * [`ImageViewUsageCreateInfo::usage`]
  * [`ImageStencilUsageCreateInfo::stencil_usage`]
  * [`ImageCreateInfo::usage`]
specify intended usage of an image, and are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageUsageFlagBits {
    VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
    VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
    VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
    VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
    VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
    VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR = 0x00000400,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR = 0x00000800,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR = 0x00001000,
#endif
  // Provided by VK_EXT_fragment_density_map
    VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x00000200,
  // Provided by VK_KHR_fragment_shading_rate
    VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00000100,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR = 0x00002000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR = 0x00004000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR = 0x00008000,
#endif
  // Provided by VK_HUAWEI_invocation_mask
    VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI = 0x00040000,
  // Provided by VK_NV_shading_rate_image
    VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV = VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR,
} VkImageUsageFlagBits;
```

# Description
- [`TRANSFER_SRC`] specifies that the image  **can**  be used as the source of a transfer command.
- [`TRANSFER_DST`] specifies that the image  **can**  be used as the destination of a transfer command.
- [`SAMPLED`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for occupying a [`DescriptorSet`] slot either of type `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and be sampled by a shader.
- [`STORAGE`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for occupying a [`DescriptorSet`] slot of type `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
- [`COLOR_ATTACHMENT`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for use as a color or resolve attachment in a [`Framebuffer`].
- [`DEPTH_STENCIL_ATTACHMENT`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for use as a depth/stencil or depth/stencil resolve attachment in a [`Framebuffer`].
- [`TRANSIENT_ATTACHMENT`] specifies that implementations  **may**  support using [memory allocations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory) with the `VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT` to back an image with this usage. This bit  **can**  be set for any image that  **can**  be used to create a [`ImageView`] suitable for use as a color, resolve, depth/stencil, or input attachment.
- [`INPUT_ATTACHMENT`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for occupying [`DescriptorSet`] slot of type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`; be read from a shader as an input attachment; and be used as an input attachment in a framebuffer.
- [`FRAGMENT_DENSITY_MAP_EXT`] specifies that the image  **can**  be used to create a [`ImageView`] suitable for use as a [fragment density map image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops).
- [`FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`] specifies     that the image  **can**  be used to create a [`ImageView`] suitable for     use as a     [fragment shading rate     attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) or     [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image)
- [`VIDEO_DECODE_DST_KHR`] specifies that [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations) **can**  use the image as an output picture for video decode operations.
- [`VIDEO_DECODE_SRC_KHR`] is reserved for future use.
- [`VIDEO_DECODE_DPB_KHR`] specifies that [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations) **can**  use the image as a [DPB Video Picture Resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources), representing a [reference picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture). If an implementation requires separate allocations for DPB and decode output, indicating this by returning `VK_ERROR_FORMAT_NOT_SUPPORTED` to any [`get_physical_device_video_format_properties_khr`] call with both [`VIDEO_DECODE_DPB_KHR`] and [`VIDEO_DECODE_DST_KHR`] usage bits, then [`VIDEO_DECODE_DPB_KHR`] **must**  not be combined with any other VK_IMAGE_USAGE_* flags. Otherwise, [`VIDEO_DECODE_DPB_KHR`] **must**  be combined with [`VIDEO_DECODE_DST_KHR`], if the DPB image is required to coincide with the decoded output picture. In the case where DPB coincides with the decoded output picture, image resources  **can**  be used as [reference pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) only after acting as targets for video decode operations, where its image view  **must**  be set to both [`VideoDecodeInfoKHR`]::`pSetupReferenceSlot->pPictureResource->imageViewBinding` and [`VideoDecodeInfoKHR`]::`dstPictureResource.imageViewBinding`.
- [`VIDEO_ENCODE_SRC_KHR`] specifies that the image  **can**  be used as an [input picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) for [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations).
- [`VIDEO_ENCODE_DST_KHR`] is reserved for future use.
- [`VIDEO_ENCODE_DPB_KHR`] specifies that [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations) **can**  use the image as an output to hold a [reconstructed picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) that can subsequently act as an input [reference picture](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).

# Related
- [`crate::vulkan1_0`]
- [`ImageUsageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        