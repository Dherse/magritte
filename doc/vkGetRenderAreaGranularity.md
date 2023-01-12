[vkGetRenderAreaGranularity](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRenderAreaGranularity.html) - Returns the granularity for optimal render area

# C Specifications
To query the render area granularity, call:
```c
// Provided by VK_VERSION_1_0
void vkGetRenderAreaGranularity(
    VkDevice                                    device,
    VkRenderPass                                renderPass,
    VkExtent2D*                                 pGranularity);
```

# Parameters
- [`device`] is the logical device that owns the render pass.
- [`render_pass`] is a handle to a render pass.
- [`p_granularity`] is a pointer to a [`Extent2D`] structure in which the granularity is returned.

# Description
The conditions leading to an optimal `renderArea` are:
- the `offset.x` member in `renderArea` is a multiple of the `width` member of the returned [`Extent2D`] (the horizontal granularity).
- the `offset.y` member in `renderArea` is a multiple of the `height` member of the returned [`Extent2D`] (the vertical granularity).
- either the `extent.width` member in `renderArea` is a multiple of the horizontal granularity or `offset.x`+`extent.width` is equal to the `width` of the `framebuffer` in the [`RenderPassBeginInfo`].
- either the `extent.height` member in `renderArea` is a multiple of the vertical granularity or `offset.y`+`extent.height` is equal to the `height` of the `framebuffer` in the [`RenderPassBeginInfo`].
Subpass dependencies are not affected by the render area, and apply to the
entire image subresources attached to the framebuffer as specified in the
description of [automatic layout
transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-layout-transitions).
Similarly, pipeline barriers are valid even if their effect extends outside
the render area.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`render_pass`] **must**  be a valid [`RenderPass`] handle
-  [`p_granularity`] **must**  be a valid pointer to a [`Extent2D`] structure
-  [`render_pass`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Extent2D`]
- [`RenderPass`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        