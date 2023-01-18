[vkCmdSetViewportShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportShadingRatePaletteNV.html) - Set shading rate image palettes dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the per-viewport shading
rate image palettes, call:
```c
// Provided by VK_NV_shading_rate_image
void vkCmdSetViewportShadingRatePaletteNV(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstViewport,
    uint32_t                                    viewportCount,
    const VkShadingRatePaletteNV*               pShadingRatePalettes);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_viewport`] is the index of the first viewport whose shading rate palette is updated by the command.
- [`viewport_count`] is the number of viewports whose shading rate palettes are updated by the command.
- [`p_shading_rate_palettes`] is a pointer to an array of [`ShadingRatePaletteNV`] structures defining the palette for each viewport.

# Description
This command sets the per-viewport shading rate image palettes for
subsequent drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportShadingRateImageStateCreateInfoNV`]::[`p_shading_rate_palettes`]
values used to create the currently active pipeline.
## Valid Usage
-    The [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shadingRateImage) feature  **must**  be enabled
-    The sum of [`first_viewport`] and [`viewport_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`first_viewport`] **must**  be `0`
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`viewport_count`] **must**  be `1`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_shading_rate_palettes`] **must**  be a valid pointer to an array of [`viewport_count`] valid [`ShadingRatePaletteNV`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`viewport_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_shading_rate_image`]
- [`CommandBuffer`]
- [`ShadingRatePaletteNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        