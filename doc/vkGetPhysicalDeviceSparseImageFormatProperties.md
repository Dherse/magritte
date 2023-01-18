[vkGetPhysicalDeviceSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html) - Retrieve properties of an image format applied to sparse images

# C Specifications
[`get_physical_device_sparse_image_format_properties`] returns an array of
[`SparseImageFormatProperties`].
Each element will describe properties for one set of image aspects that are
bound simultaneously in the image.
This is usually one element for each aspect in the image, but for
interleaved depth/stencil images there is only one element describing the
combined aspects.
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceSparseImageFormatProperties(
    VkPhysicalDevice                            physicalDevice,
    VkFormat                                    format,
    VkImageType                                 type,
    VkSampleCountFlagBits                       samples,
    VkImageUsageFlags                           usage,
    VkImageTiling                               tiling,
    uint32_t*                                   pPropertyCount,
    VkSparseImageFormatProperties*              pProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the sparse image format properties.
- [`format`] is the image format.
- [`type_`] is the dimensionality of image.
- [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples per texel.
- [`usage`] is a bitmask describing the intended usage of the image.
- [`tiling`] is the tiling arrangement of the texel blocks in memory.
- [`p_property_count`] is a pointer to an integer related to the number of sparse format properties available or queried, as described below.
- [`p_properties`] is either `NULL` or a pointer to an array of [`SparseImageFormatProperties`] structures.

# Description
If [`p_properties`] is `NULL`, then the number of sparse format properties
available is returned in [`p_property_count`].
Otherwise, [`p_property_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_properties`] array, and on return the
variable is overwritten with the number of structures actually written to
[`p_properties`].
If [`p_property_count`] is less than the number of sparse format properties
available, at most [`p_property_count`] structures will be written.If `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` is not supported for the given
arguments, [`p_property_count`] will be set to zero upon return, and no data
will be written to [`p_properties`].Multiple aspects are returned for depth/stencil images that are implemented
as separate planes by the implementation.
The depth and stencil data planes each have unique
[`SparseImageFormatProperties`] data.Depth/stencil images with depth and stencil data interleaved into a single
plane will return a single [`SparseImageFormatProperties`] structure
with the `aspectMask` set to `VK_IMAGE_ASPECT_DEPTH_BIT` |
`VK_IMAGE_ASPECT_STENCIL_BIT`.
## Valid Usage
-  [`samples`] **must**  be a bit value that is set in [`ImageFormatProperties::sample_counts`] returned by [`get_physical_device_image_format_properties`] with [`format`], [`type_`], [`tiling`], and [`usage`] equal to those in this command and `flags` equal to the value that is set in [`ImageCreateInfo::flags`] when the image is created

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`format`] **must**  be a valid [`Format`] value
-  [`type_`] **must**  be a valid [`ImageType`] value
-  [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`tiling`] **must**  be a valid [`ImageTiling`] value
-  [`p_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_property_count`] is not `0`, and [`p_properties`] is not `NULL`, [`p_properties`] **must**  be a valid pointer to an array of [`p_property_count`][`SparseImageFormatProperties`] structures

# Related
- [`crate::vulkan1_0`]
- [`Format`]
- [`ImageTiling`]
- [`ImageType`]
- [`ImageUsageFlags`]
- [`PhysicalDevice`]
- [`SampleCountFlagBits`]
- [`SparseImageFormatProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        