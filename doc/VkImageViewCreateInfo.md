[VkImageViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateInfo.html) - Structure specifying parameters of a newly created image view

# C Specifications
The [`ImageViewCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageViewCreateInfo {
    VkStructureType            sType;
    const void*                pNext;
    VkImageViewCreateFlags     flags;
    VkImage                    image;
    VkImageViewType            viewType;
    VkFormat                   format;
    VkComponentMapping         components;
    VkImageSubresourceRange    subresourceRange;
} VkImageViewCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`ImageViewCreateFlagBits`] describing additional parameters of the image view.
- [`image`] is a [`Image`] on which the view will be created.
- [`view_type`] is a [`ImageViewType`] value specifying the type of the image view.
- [`format`] is a [`Format`] describing the format and type used to interpret texel blocks in the image.
- [`components`] is a [`ComponentMapping`] structure specifying a remapping of color components (or of depth or stencil components after they have been converted into color components).
- [`subresource_range`] is a [`ImageSubresourceRange`] structure selecting the set of mipmap levels and array layers to be accessible to the view.

# Description
Some of the [`image`] creation parameters are inherited by the view.
In particular, image view creation inherits the implicit parameter
`usage` specifying the allowed usages of the image view that, by
default, takes the value of the corresponding `usage` parameter
specified in [`ImageCreateInfo`] at image creation time.
The implicit `usage` **can**  be overriden by adding a
[`ImageViewUsageCreateInfo`] structure to the [`p_next`] chain, but the
view usage  **must**  be a subset of the image usage.
If [`image`] has a depth-stencil format and was created with a
[`ImageStencilUsageCreateInfo`] structure included in the [`p_next`]
chain of [`ImageCreateInfo`], the usage is calculated based on the
`subresource.aspectMask` provided:
- If `aspectMask` includes only `VK_IMAGE_ASPECT_STENCIL_BIT`, the implicit `usage` is equal to [`ImageStencilUsageCreateInfo::stencil_usage`].
- If `aspectMask` includes only `VK_IMAGE_ASPECT_DEPTH_BIT`, the implicit `usage` is equal to [`ImageCreateInfo::usage`].
- If both aspects are included in `aspectMask`, the implicit `usage` is equal to the intersection of [`ImageCreateInfo::usage`] and [`ImageStencilUsageCreateInfo::stencil_usage`].
If [`image`] was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`
flag,
and if the [`format`] of the image is not
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
[`format`] **can**  be different from the image’s format, but if
[`image`] was created without the
`VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag and
they are not equal they  **must**  be *compatible*.
Image format compatibility is defined in the
[Format Compatibility Classes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility-classes) section.
Views of compatible formats will have the same mapping between texel
coordinates and memory locations irrespective of the [`format`], with only
the interpretation of the bit pattern changing.If [`image`] was created with the
`VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag, [`format`] **must**  be *compatible* with the image’s format as described above, or  **must** 
be an uncompressed format in which case it  **must**  be *size-compatible* with
the image’s format, as defined for
[copying data between images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-images-format-size-compatibility).
In this case, the resulting image view’s texel dimensions equal the
dimensions of the selected mip level divided by the compressed texel block
size and rounded up.The [`ComponentMapping`][`components`] member describes a remapping
from components of the image to components of the vector returned by shader
image instructions.
This remapping  **must**  be the identity swizzle for storage image descriptors,
input attachment descriptors,
framebuffer attachments, and any [`ImageView`] used with a combined
image sampler that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion).If the image view is to be used with a sampler which supports
[sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion), an *identically
defined object* of type [`SamplerYcbcrConversion`] to that used to
create the sampler  **must**  be passed to [`create_image_view`] in a
[`SamplerYcbcrConversionInfo`] included in the [`p_next`] chain of
[`ImageViewCreateInfo`].
Conversely, if a [`SamplerYcbcrConversion`] object is passed to
[`create_image_view`], an identically defined
[`SamplerYcbcrConversion`] object  **must**  be used when sampling the image.If the image has a
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)[`format`] and
`subresourceRange.aspectMask` is `VK_IMAGE_ASPECT_COLOR_BIT`,
    and it was created with `usage` value containing flags other than
    `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`,
    `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`
    , `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`,
    `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`,
then the [`format`] **must**  be identical to the image [`format`], and the
sampler to be used with the image view  **must**  enable
[sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion).If the image has a
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)[`format`] and
the [`image`] has been created with a `usage` value containing any of
the `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`,
`VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR`, and
`VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` flags, then all of the
[video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations) would ignore the
[`SamplerYcbcrConversionInfo`] structure and/or
[sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) object, associated
with the image view.
If the image has a
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)[`format`] and
the [`image`] has been created with a `usage` value containing any of
the `VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR`,
`VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, and
`VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR` flags, then all of the
[video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations) would ignore the
[`SamplerYcbcrConversionInfo`] structure and/or
[sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) object, associated
with the image view.If [`image`] was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`
and the image has a
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)[`format`],
and if `subresourceRange.aspectMask` is
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or
`VK_IMAGE_ASPECT_PLANE_2_BIT`, [`format`] **must**  be
[compatible](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes) with the corresponding plane of the
image, and the sampler to be used with the image view  **must**  not enable
[sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion).
The `width` and `height` of the single-plane image view  **must**  be
derived from the multi-planar image’s dimensions in the manner listed for
[plane compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes) for the plane.Any view of an image plane will have the same mapping between texel
coordinates and memory locations as used by the components of the color
aspect, subject to the formulae relating texel coordinates to
lower-resolution planes as described in [Chroma Reconstruction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction).
That is, if an R or B plane has a reduced resolution relative to the G plane
of the multi-planar image, the image view operates using the (*u<sub>plane</sub>*,
*v<sub>plane</sub>*) unnormalized coordinates of the reduced-resolution plane, and
these coordinates access the same memory locations as the (*u<sub>color</sub>*,
*v<sub>color</sub>*) unnormalized coordinates of the color aspect for which chroma
reconstruction operations operate on the same (*u<sub>plane</sub>*, *v<sub>plane</sub>*) or
(*i<sub>plane</sub>*, *j<sub>plane</sub>*) coordinates.
## Valid Usage
-    If [`image`] was not created with `VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT` then [`view_type`] **must**  not be `VK_IMAGE_VIEW_TYPE_CUBE` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`
-    If the [image cube map arrays](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-imageCubeArray) feature is not enabled, [`view_type`] **must**  not be `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`
-    If [`image`] was created with `VK_IMAGE_TYPE_3D` but without `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` set then [`view_type`] **must**  not be `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`
-    If [`image`] was created with `VK_IMAGE_TYPE_3D` and [`view_type`] is `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY` then `subresourceRange.levelCount` **must**  be 1
-    If [`image`] was created with `VK_IMAGE_TYPE_3D` and [`view_type`] is `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY` then [`ImageCreateInfo`]::[`flags`] **must**  not contain any of `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, and `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`
-    If [`image`] was created with a `samples` value not equal to `VK_SAMPLE_COUNT_1_BIT` then [`view_type`] **must**  be either `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`
-  [`image`] **must**  have been created with a `usage` value containing at least one of the usages defined in the [valid image usage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#valid-imageview-imageusage) list for image views
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) of the resultant image view  **must**  contain at least one bit
-    If `usage` contains `VK_IMAGE_USAGE_SAMPLED_BIT`, then the [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) of the resultant image view  **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`
-    If `usage` contains `VK_IMAGE_USAGE_STORAGE_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT`
-    If `usage` contains `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If `usage` contains `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If `usage` contains `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`, if the image is created with `VK_IMAGE_TILING_LINEAR` and the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled
-    If `usage` contains `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) must contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`, if the image is created with `VK_IMAGE_TILING_LINEAR` and the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled
-    If `usage` contains `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain at least one of `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-  `subresourceRange.baseMipLevel` **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.levelCount` is not `VK_REMAINING_MIP_LEVELS`, `subresourceRange.baseMipLevel` +  `subresourceRange.levelCount` **must**  be less than or equal to the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    If [`image`] was created with `usage` containing `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`, `subresourceRange.levelCount` **must**  be `1`
-    If [`image`] is not a 3D image created with `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` set, or [`view_type`] is not `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`, `subresourceRange.baseArrayLayer` **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, [`image`] is not a 3D image created with `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` set, or [`view_type`] is not `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`, `subresourceRange.layerCount` **must**  be non-zero and `subresourceRange.baseArrayLayer` +  `subresourceRange.layerCount` **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If [`image`] is a 3D image created with `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` set, and [`view_type`] is `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`, `subresourceRange.baseArrayLayer` **must**  be less than the depth computed from `baseMipLevel` and `extent.depth` specified in [`ImageCreateInfo`] when [`image`] was created, according to the formula defined in [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing)
-    If `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, [`image`] is a 3D image created with `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` set, and [`view_type`] is `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`, `subresourceRange.layerCount` **must**  be non-zero and `subresourceRange.baseArrayLayer` +  `subresourceRange.layerCount` **must**  be less than or equal to the depth computed from `baseMipLevel` and `extent.depth` specified in [`ImageCreateInfo`] when [`image`] was created, according to the formula defined in [Image Miplevel Sizing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-miplevel-sizing)
-    If [`image`] was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` flag, but without the `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag, and if the [`format`] of the [`image`] is not a [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) format, [`format`] **must**  be compatible with the [`format`] used to create [`image`], as defined in [Format Compatibility Classes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility-classes)
-    If [`image`] was created with the `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag, [`format`] **must**  be compatible with, or  **must**  be an uncompressed format that is size-compatible with, the [`format`] used to create [`image`]
-    If [`image`] was created with the `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag, the `levelCount` and `layerCount` members of [`subresource_range`] **must**  both be `1`
-    If [`image`] was created with the `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT` flag and [`format`] is a non-compressed format, [`view_type`] **must**  not be `VK_IMAGE_VIEW_TYPE_3D`
-    If a [`ImageFormatListCreateInfo`] structure was included in the [`p_next`] chain of the [`ImageCreateInfo`] structure used when creating [`image`] and [`ImageFormatListCreateInfo::view_format_count`] is not zero then [`format`] **must**  be one of the formats in [`ImageFormatListCreateInfo::view_formats`]
-    If [`image`] was created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` flag, if the [`format`] of the [`image`] is a [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) format, and if `subresourceRange.aspectMask` is one of `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`, then [`format`] **must**  be compatible with the [`Format`] for the plane of the [`image`][`format`] indicated by `subresourceRange.aspectMask`, as defined in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes)
-    If [`image`] was not created with the `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT` flag, or if the [`format`] of the [`image`] is a [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) format and if `subresourceRange.aspectMask` is `VK_IMAGE_ASPECT_COLOR_BIT`, [`format`] **must**  be identical to the [`format`] used to create [`image`]
-    If the image view [requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#image-views-requiring-sampler-ycbcr-conversion), the [`p_next`] chain must include a [`SamplerYcbcrConversionInfo`] structure with a conversion value other than [`crate::Handle::null`]
-    If [`format`] has a `_422` or `_420` suffix then [`image`] **must**  have been created with a width that is a multiple of 2
-    If [`format`] has a `_420` suffix then [`image`] **must**  have been created with a height that is a multiple of 2
-    If the [`p_next`] chain includes a [`SamplerYcbcrConversionInfo`] structure with a `conversion` value other than [`crate::Handle::null`], all members of [`components`] **must**  have the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
-    If the [`p_next`] chain includes a [`SamplerYcbcrConversionInfo`] structure with a `conversion` value other than [`crate::Handle::null`], [`format`] **must**  be the same used in [`SamplerYcbcrConversionCreateInfo`]::[`format`]
-    If [`image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`view_type`] **must**  be compatible with the type of [`image`] as shown in the [view type compatibility table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-compatibility)
-    If [`image`] has an [external format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats), [`format`] **must**  be `VK_FORMAT_UNDEFINED`
-    If [`image`] has an [external format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats), the [`p_next`] chain  **must**  include a [`SamplerYcbcrConversionInfo`] structure with a `conversion` object created with the same external format as [`image`]
-    If [`image`] has an [external format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats), all members of [`components`] **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
-    If [`image`] was created with `usage` containing `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, [`view_type`] **must**  be `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`
-    If the [`shadingRateImage` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shadingRateImage) is enabled, and If [`image`] was created with `usage` containing `VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`, [`format`] **must**  be `VK_FORMAT_R8_UINT`
-    If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) is enabled, and the `usage` for the image view includes `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, then the image view’s [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-view-format-features) **must**  contain `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
-    If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) is enabled, the `usage` for the image view includes `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`, and [`layeredShadingRateAttachments`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-layeredShadingRateAttachments) is `VK_FALSE`, `subresourceRange.layerCount` **must**  be `1`
-    If [dynamic fragment density map](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentDensityMapDynamic) feature is not enabled, [`flags`] **must**  not contain `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`
-    If [deferred fragment density map](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentDensityMapDeferred) feature is not enabled, [`flags`] **must**  not contain `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT`
-    If [`flags`] contains `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT`, [`flags`] **must**  not contain `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`
-    If [`image`] was created with [`flags`] containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT` and `usage` containing `VK_IMAGE_USAGE_SAMPLED_BIT`, `subresourceRange.layerCount` **must**  be less than or equal to [[`PhysicalDeviceFragmentDensityMap2PropertiesEXT::max_subsampled_array_layers`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSubsampledArrayLayers)
-    If the [`invocationMask` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-invocationMask) is enabled, and if [`image`] was created with `usage` containing `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI`, [`format`] **must**  be `VK_FORMAT_R8_UINT`
-    If [`flags`] does not contain `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` and [`image`] was created with `usage` containing `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`, its [`flags`] **must**  not contain any of `VK_IMAGE_CREATE_PROTECTED_BIT`, `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, or `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`
-    If the [`p_next`] chain includes a [`ImageViewUsageCreateInfo`] structure, and [`image`] was not created with a [`ImageStencilUsageCreateInfo`] structure included in the [`p_next`] chain of [`ImageCreateInfo`], its `usage` member  **must**  not include any bits that were not set in the `usage` member of the [`ImageCreateInfo`] structure used to create [`image`]
-    If the [`p_next`] chain includes a [`ImageViewUsageCreateInfo`] structure, [`image`] was created with a [`ImageStencilUsageCreateInfo`] structure included in the [`p_next`] chain of [`ImageCreateInfo`], and `subresourceRange.aspectMask` includes `VK_IMAGE_ASPECT_STENCIL_BIT`, the `usage` member of the [`ImageViewUsageCreateInfo`] structure  **must**  not include any bits that were not set in the `usage` member of the [`ImageStencilUsageCreateInfo`] structure used to create [`image`]
-    If the [`p_next`] chain includes a [`ImageViewUsageCreateInfo`] structure, [`image`] was created with a [`ImageStencilUsageCreateInfo`] structure included in the [`p_next`] chain of [`ImageCreateInfo`], and `subresourceRange.aspectMask` includes bits other than `VK_IMAGE_ASPECT_STENCIL_BIT`, the `usage` member of the [`ImageViewUsageCreateInfo`] structure  **must**  not include any bits that were not set in the `usage` member of the [`ImageCreateInfo`] structure used to create [`image`]
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_1D`, `VK_IMAGE_VIEW_TYPE_2D`, or `VK_IMAGE_VIEW_TYPE_3D`; and `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, then `subresourceRange.layerCount` **must**  be 1
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_1D`, `VK_IMAGE_VIEW_TYPE_2D`, or `VK_IMAGE_VIEW_TYPE_3D`; and `subresourceRange.layerCount` is `VK_REMAINING_ARRAY_LAYERS`, then the remaining number of layers  **must**  be 1
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_CUBE` and `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, `subresourceRange.layerCount` **must**  be `6`
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY` and `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, `subresourceRange.layerCount` **must**  be a multiple of `6`
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_CUBE` and `subresourceRange.layerCount` is `VK_REMAINING_ARRAY_LAYERS`, the remaining number of layers  **must**  be `6`
-    If [`view_type`] is `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY` and `subresourceRange.layerCount` is `VK_REMAINING_ARRAY_LAYERS`, the remaining number of layers  **must**  be a multiple of `6`
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::image_view_format_swizzle`] is `VK_FALSE`, all elements of [`components`] **must**  have the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::image_view_format_reinterpretation`] is `VK_FALSE`, the [`Format`] in [`format`] **must**  not contain a different number of components, or a different number of bits in each component, than the format of the [`Image`] in [`image`]
-    If [`image`] was created with `usage` containing `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_DECODE_SRC_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`, then the [`view_type`] **must**  be `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY` and all members of [`components`] **must**  have the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
-    If [`image`] was created with `usage` containing `VK_IMAGE_USAGE_VIDEO_ENCODE_DST_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`, then the [`view_type`] **must**  be `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY` and all members of [`components`] **must**  have the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`ImageViewAstcDecodeModeEXT`], [`ImageViewMinLodCreateInfoEXT`], [`ImageViewUsageCreateInfo`], [`SamplerYcbcrConversionInfo`], [`VideoDecodeH264ProfileEXT`], [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`], [`VideoProfileKHR`], or [`VideoProfilesKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`ImageViewCreateFlagBits`] values
-  [`image`] **must**  be a valid [`Image`] handle
-  [`view_type`] **must**  be a valid [`ImageViewType`] value
-  [`format`] **must**  be a valid [`Format`] value
-  [`components`] **must**  be a valid [`ComponentMapping`] structure
-  [`subresource_range`] **must**  be a valid [`ImageSubresourceRange`] structure

# Related
- [`crate::vulkan1_0`]
- [`ComponentMapping`]
- [`Format`]
- [`Image`]
- [`ImageSubresourceRange`]
- [VkImageViewCreateFlags]()
- [`ImageViewType`]
- [`StructureType`]
- [`create_image_view`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        