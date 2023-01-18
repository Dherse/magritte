[vkGetPhysicalDeviceVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html) - Query supported Video Decode and Encode image formats

# C Specifications
To enumerate the supported output, input and DPB image formats for a
specific codec operation and video profile, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceVideoFormatInfoKHR*   pVideoFormatInfo,
    uint32_t*                                   pVideoFormatPropertyCount,
    VkVideoFormatPropertiesKHR*                 pVideoFormatProperties);
```

# Parameters
- [`physical_device`] is the physical device being queried.
- [`p_video_format_info`] is a pointer to a [`PhysicalDeviceVideoFormatInfoKHR`] structure specifying the codec and video profile for which information is returned.
- [`p_video_format_property_count`] is a pointer to an integer related to the number of video format properties available or queried, as described below.
- [`p_video_format_properties`] is a pointer to an array of [`VideoFormatPropertiesKHR`] structures in which supported formats are returned.

# Description
If [`p_video_format_properties`] is `NULL`, then the number of video format
properties supported for the given [`physical_device`] is returned in
[`p_video_format_property_count`].
Otherwise, [`p_video_format_property_count`] **must**  point to a variable set by
the user to the number of elements in the [`p_video_format_properties`]
array, and on return the variable is overwritten with the number of values
actually written to [`p_video_format_properties`].
If the value of [`p_video_format_property_count`] is less than the number of
video format properties supported, at most [`p_video_format_property_count`]
values will be written to [`p_video_format_properties`], and
`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
indicate that not all the available values were returned.If an implementation reports
`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is
supported but
`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is not
supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
for video format properties for decode DPB or output, `imageUsage` **must** 
have both `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` and
`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` set.
Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.If an implementation reports
`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR` is
supported but
`VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR` is not
supported in [`VideoDecodeCapabilitiesKHR::flags`], then to query
for video format properties for decode DPB, `imageUsage` **must**  have
`VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` set and
`VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR` not set.
Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
Similarly, to query for video format properties for decode output,
`imageUsage` **must**  have `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
set and `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` not set.
Otherwise, the call will fail with `VK_ERROR_FORMAT_NOT_SUPPORTED`.
## Valid Usage
-    The `imageUsage` enum of [`PhysicalDeviceVideoFormatInfoKHR`] **must**  contain at least one of the following video image usage bit(s): `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR`, `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`, or `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_video_format_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceVideoFormatInfoKHR`] structure
-  [`p_video_format_property_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_video_format_property_count`] is not `0`, and [`p_video_format_properties`] is not `NULL`, [`p_video_format_properties`] **must**  be a valid pointer to an array of [`p_video_format_property_count`][`VideoFormatPropertiesKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_EXTENSION_NOT_PRESENT`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`VK_KHR_video_queue`]
- [`PhysicalDevice`]
- [`PhysicalDeviceVideoFormatInfoKHR`]
- [`VideoFormatPropertiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        