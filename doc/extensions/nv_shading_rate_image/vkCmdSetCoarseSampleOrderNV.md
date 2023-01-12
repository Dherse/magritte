[vkCmdSetCoarseSampleOrderNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoarseSampleOrderNV.html) - Set order of coverage samples for coarse fragments dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the order of coverage
samples in fragments larger than one pixel, call:
```c
// Provided by VK_NV_shading_rate_image
void vkCmdSetCoarseSampleOrderNV(
    VkCommandBuffer                             commandBuffer,
    VkCoarseSampleOrderTypeNV                   sampleOrderType,
    uint32_t                                    customSampleOrderCount,
    const VkCoarseSampleOrderCustomNV*          pCustomSampleOrders);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`sample_order_type`] specifies the mechanism used to order coverage samples in fragments larger than one pixel.
- [`custom_sample_order_count`] specifies the number of custom sample orderings to use when ordering coverage samples.
- [`p_custom_sample_orders`] is a pointer to an array of [`CoarseSampleOrderCustomNV`] structures, each structure specifying the coverage sample order for a single combination of fragment area and coverage sample count.

# Description
If [`sample_order_type`] is `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, the
coverage sample order used for any combination of fragment area and coverage
sample count not enumerated in [`p_custom_sample_orders`] will be identical
to that used for `VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.This command sets the order of coverage samples for subsequent drawing
commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] values used to
create the currently active pipeline.
## Valid Usage
-    If [`sample_order_type`] is not `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, `customSamplerOrderCount` **must**  be `0`
-    The array [`p_custom_sample_orders`] **must**  not contain two structures with matching values for both the `shadingRate` and `sampleCount` members

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`sample_order_type`] **must**  be a valid [`CoarseSampleOrderTypeNV`] value
-    If [`custom_sample_order_count`] is not `0`, [`p_custom_sample_orders`] **must**  be a valid pointer to an array of [`custom_sample_order_count`] valid [`CoarseSampleOrderCustomNV`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_shading_rate_image`]
- [`CoarseSampleOrderCustomNV`]
- [`CoarseSampleOrderTypeNV`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        