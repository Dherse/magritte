[VkRectLayerKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html) - Structure containing a rectangle, including layer, changed by vkQueuePresentKHR for a given VkImage

# C Specifications
The [`RectLayerKHR`] structure is defined as:
```c
// Provided by VK_KHR_incremental_present
typedef struct VkRectLayerKHR {
    VkOffset2D    offset;
    VkExtent2D    extent;
    uint32_t      layer;
} VkRectLayerKHR;
```

# Members
- [`offset`] is the origin of the rectangle, in pixels.
- [`extent`] is the size of the rectangle, in pixels.
- [`layer`] is the layer of the image. For images with only one layer, the value of [`layer`] **must**  be 0.

# Description
Some platforms allow the size of a surface to change, and then scale the
pixels of the image to fit the surface.
[`RectLayerKHR`] specifies pixels of the swapchain’s image(s), which
will be constant for the life of the swapchain.
## Valid Usage
-    The sum of [`offset`] and [`extent`], after being transformed according to the `preTransform` member of the [`SwapchainCreateInfoKHR`] structure,  **must**  be no greater than the `imageExtent` member of the [`SwapchainCreateInfoKHR`] structure passed to [`create_swapchain_khr`]
-  [`layer`] **must**  be less than the `imageArrayLayers` member of the [`SwapchainCreateInfoKHR`] structure passed to [`create_swapchain_khr`]

# Related
- [`khr_incremental_present`]
- [`Extent2D`]
- [`Offset2D`]
- [`PresentRegionKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        