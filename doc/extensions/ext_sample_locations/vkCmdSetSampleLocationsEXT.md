[vkCmdSetSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html) - Set sample locations dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the sample locations used
for rasterization, call:
```c
// Provided by VK_EXT_sample_locations
void vkCmdSetSampleLocationsEXT(
    VkCommandBuffer                             commandBuffer,
    const VkSampleLocationsInfoEXT*             pSampleLocationsInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_sample_locations_info`] is the sample locations state to set.

# Description
This command sets the custom sample locations for subsequent drawing
commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`], and when the
[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`]
property of the bound graphics pipeline is `VK_TRUE`.
Otherwise, this state is specified by the
[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_info`]
values used to create the currently active pipeline.
## Valid Usage
-    The `sampleLocationsPerPixel` member of [`p_sample_locations_info`] **must**  equal the `rasterizationSamples` member of the [`PipelineMultisampleStateCreateInfo`] structure the bound graphics pipeline has been created with
-    If [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is `VK_FALSE` then the current render pass  **must**  have been begun by specifying a [`RenderPassSampleLocationsBeginInfoEXT`] structure whose `pPostSubpassSampleLocations` member contains an element with a `subpassIndex` matching the current subpass index and the `sampleLocationsInfo` member of that element  **must**  match the sample locations state pointed to by [`p_sample_locations_info`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_sample_locations_info`] **must**  be a valid pointer to a valid [`SampleLocationsInfoEXT`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_sample_locations`]
- [`CommandBuffer`]
- [`SampleLocationsInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        