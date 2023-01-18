[VkSubpassSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html) - Structure specifying the sample locations state to use for layout transitions of attachments performed after a given subpass

# C Specifications
The [`SubpassSampleLocationsEXT`] structure is defined as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkSubpassSampleLocationsEXT {
    uint32_t                    subpassIndex;
    VkSampleLocationsInfoEXT    sampleLocationsInfo;
} VkSubpassSampleLocationsEXT;
```

# Members
- [`subpass_index`] is the index of the subpass for which the sample locations state is provided.
- [`sample_locations_info`] is the sample locations state to use for the layout transition of the depth/stencil attachment away from the image layout the attachment is used with in the subpass specified in [`subpass_index`].

# Description
If the image referenced by the depth/stencil attachment used in the subpass
identified by [`subpass_index`] was not created with
`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` or if the
subpass does not use a depth/stencil attachment, and
[`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`]
is [`TRUE`] then the values specified in [`sample_locations_info`] are
ignored.
## Valid Usage
-  [`subpass_index`] **must**  be less than the `subpassCount` specified in [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was created with

## Valid Usage (Implicit)
-  [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure

# Related
- [`VK_EXT_sample_locations`]
- [`RenderPassSampleLocationsBeginInfoEXT`]
- [`SampleLocationsInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        