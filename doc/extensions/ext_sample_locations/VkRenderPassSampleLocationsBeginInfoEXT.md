[VkRenderPassSampleLocationsBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) - Structure specifying sample locations to use for the layout transition of custom sample locations compatible depth/stencil attachments

# C Specifications
The image layout of the depth aspect of a depth/stencil attachment referring
to an image created with
`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent
on the last sample locations used to render to the image subresource, thus
preserving the contents of such depth/stencil attachments across subpass
boundaries requires the application to specify these sample locations
whenever a layout transition of the attachment  **may**  occur.
This information  **can**  be provided by adding a
[`RenderPassSampleLocationsBeginInfoEXT`] structure to the [`p_next`]
chain of [`RenderPassBeginInfo`].The [`RenderPassSampleLocationsBeginInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkRenderPassSampleLocationsBeginInfoEXT {
    VkStructureType                          sType;
    const void*                              pNext;
    uint32_t                                 attachmentInitialSampleLocationsCount;
    const VkAttachmentSampleLocationsEXT*    pAttachmentInitialSampleLocations;
    uint32_t                                 postSubpassSampleLocationsCount;
    const VkSubpassSampleLocationsEXT*       pPostSubpassSampleLocations;
} VkRenderPassSampleLocationsBeginInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attachment_initial_sample_locations_count`] is the number of elements in the [`attachment_initial_sample_locations`] array.
- [`attachment_initial_sample_locations`] is a pointer to an array of [`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures specifying the attachment indices and their corresponding sample location state. Each element of [`attachment_initial_sample_locations`] **can**  specify the sample location state to use in the automatic layout transition performed to transition a depth/stencil attachment from the initial layout of the attachment to the image layout specified for the attachment in the first subpass using it.
- [`post_subpass_sample_locations_count`] is the number of elements in the [`post_subpass_sample_locations`] array.
- [`post_subpass_sample_locations`] is a pointer to an array of [`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`] structures specifying the subpass indices and their corresponding sample location state. Each element of [`post_subpass_sample_locations`] **can**  specify the sample location state to use in the automatic layout transition performed to transition the depth/stencil attachment used by the specified subpass to the image layout specified in a dependent subpass or to the final layout of the attachment in case the specified subpass is the last subpass using that attachment. In addition, if [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is [`FALSE`], each element of [`post_subpass_sample_locations`] **must**  specify the sample location state that matches the sample locations used by all pipelines that will be bound to a command buffer during the specified subpass. If `variableSampleLocations` is [`TRUE`], the sample locations used for rasterization do not depend on [`post_subpass_sample_locations`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`
-    If [`attachment_initial_sample_locations_count`] is not `0`, [`attachment_initial_sample_locations`] **must**  be a valid pointer to an array of [`attachment_initial_sample_locations_count`] valid [`AttachmentSampleLocationsEXT`] structures
-    If [`post_subpass_sample_locations_count`] is not `0`, [`post_subpass_sample_locations`] **must**  be a valid pointer to an array of [`post_subpass_sample_locations_count`] valid [`SubpassSampleLocationsEXT`] structures

# Related
- [`VK_EXT_sample_locations`]
- [`AttachmentSampleLocationsEXT`]
- [`StructureType`]
- [`SubpassSampleLocationsEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        