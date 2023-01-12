[VkAttachmentSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html) - Structure specifying the sample locations state to use in the initial layout transition of attachments

# C Specifications
The [`AttachmentSampleLocationsEXT`] structure is defined as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkAttachmentSampleLocationsEXT {
    uint32_t                    attachmentIndex;
    VkSampleLocationsInfoEXT    sampleLocationsInfo;
} VkAttachmentSampleLocationsEXT;
```

# Members
- [`attachment_index`] is the index of the attachment for which the sample locations state is provided.
- [`sample_locations_info`] is the sample locations state to use for the layout transition of the given attachment from the initial layout of the attachment to the image layout specified for the attachment in the first subpass using it.

# Description
If the image referenced by the framebuffer attachment at index
[`attachment_index`] was not created with
`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` then the
values specified in [`sample_locations_info`] are ignored.
## Valid Usage
-  [`attachment_index`] **must**  be less than the `attachmentCount` specified in [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was created with

## Valid Usage (Implicit)
-  [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure

# Related
- [`ext_sample_locations`]
- [`RenderPassSampleLocationsBeginInfoEXT`]
- [`SampleLocationsInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        