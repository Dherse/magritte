[VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) - Structure describing capabilities of a mode and plane combination

# C Specifications
The [`DisplayPlaneCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayPlaneCapabilitiesKHR {
    VkDisplayPlaneAlphaFlagsKHR    supportedAlpha;
    VkOffset2D                     minSrcPosition;
    VkOffset2D                     maxSrcPosition;
    VkExtent2D                     minSrcExtent;
    VkExtent2D                     maxSrcExtent;
    VkOffset2D                     minDstPosition;
    VkOffset2D                     maxDstPosition;
    VkExtent2D                     minDstExtent;
    VkExtent2D                     maxDstExtent;
} VkDisplayPlaneCapabilitiesKHR;
```

# Members
- [`supported_alpha`] is a bitmask of [`DisplayPlaneAlphaFlagBitsKHR`] describing the supported alpha blending modes.
- [`min_src_position`] is the minimum source rectangle offset supported by this plane using the specified mode.
- [`max_src_position`] is the maximum source rectangle offset supported by this plane using the specified mode. The `x` and `y` components of [`max_src_position`] **must**  each be greater than or equal to the `x` and `y` components of [`min_src_position`], respectively.
- [`min_src_extent`] is the minimum source rectangle size supported by this plane using the specified mode.
- [`max_src_extent`] is the maximum source rectangle size supported by this plane using the specified mode.
- [`min_dst_position`], [`max_dst_position`], [`min_dst_extent`], [`max_dst_extent`] all have similar semantics to their corresponding `*Src*` equivalents, but apply to the output region within the mode rather than the input region within the source image. Unlike the `*Src*` offsets, [`min_dst_position`] and [`max_dst_position`] **may**  contain negative values.

# Description
The minimum and maximum position and extent fields describe the
implementation limits, if any, as they apply to the specified display mode
and plane.
Vendors  **may**  support displaying a subset of a swapchain’s presentable images
on the specified display plane.
This is expressed by returning [`min_src_position`], [`max_src_position`],
[`min_src_extent`], and [`max_src_extent`] values that indicate a range of
possible positions and sizes which  **may**  be used to specify the region within
the presentable images that source pixels will be read from when creating a
swapchain on the specified display mode and plane.Vendors  **may**  also support mapping the presentable images’ content to a
subset or superset of the visible region in the specified display mode.
This is expressed by returning [`min_dst_position`], [`max_dst_position`],
[`min_dst_extent`] and [`max_dst_extent`] values that indicate a range of
possible positions and sizes which  **may**  be used to describe the region
within the display mode that the source pixels will be mapped to.Other vendors  **may**  support only a 1-1 mapping between pixels in the
presentable images and the display mode.
This  **may**  be indicated by returning (0,0) for [`min_src_position`],
[`max_src_position`], [`min_dst_position`], and [`max_dst_position`], and
(display mode width, display mode height) for [`min_src_extent`],
[`max_src_extent`], [`min_dst_extent`], and [`max_dst_extent`].The value [`supported_alpha`] **must**  contain at least one valid
[`DisplayPlaneAlphaFlagBitsKHR`] bit.These values indicate the limits of the implementation’s individual fields.
Not all combinations of values within the offset and extent ranges returned
in [`DisplayPlaneCapabilitiesKHR`] are guaranteed to be supported.
Presentation requests specifying unsupported combinations  **may**  fail.

# Related
- [`khr_display`]
- [VkDisplayPlaneAlphaFlagsKHR]()
- [`DisplayPlaneCapabilities2KHR`]
- [`Extent2D`]
- [`Offset2D`]
- [`get_display_plane_capabilities_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        