[VkPresentRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html) - Structure containing rectangular region changed by vkQueuePresentKHR for a given VkImage

# C Specifications
For a given image and swapchain, the region to present is specified by the
[`PresentRegionKHR`] structure, which is defined as:
```c
// Provided by VK_KHR_incremental_present
typedef struct VkPresentRegionKHR {
    uint32_t                 rectangleCount;
    const VkRectLayerKHR*    pRectangles;
} VkPresentRegionKHR;
```

# Members
- [`rectangle_count`] is the number of rectangles in [`rectangles`], or zero if the entire image has changed and should be presented.
- [`rectangles`] is either `NULL` or a pointer to an array of [`RectLayerKHR`] structures. The [`RectLayerKHR`] structure is the framebuffer coordinates, plus layer, of a portion of a presentable image that has changed and  **must**  be presented. If non-`NULL`, each entry in [`rectangles`] is a rectangle of the given image that has changed since the last image was presented to the given swapchain. The rectangles  **must**  be specified relative to [`SurfaceCapabilitiesKHR::current_transform`], regardless of the swapchain’s `preTransform`. The presentation engine will apply the `preTransform` transformation to the rectangles, along with any further transformation it applies to the image content.

# Description
## Valid Usage (Implicit)
-    If [`rectangle_count`] is not `0`, and [`rectangles`] is not `NULL`, [`rectangles`] **must**  be a valid pointer to an array of [`rectangle_count`] valid [`RectLayerKHR`] structures

# Related
- [`khr_incremental_present`]
- [`PresentRegionsKHR`]
- [`RectLayerKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        