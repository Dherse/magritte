[VkRect2D](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRect2D.html) - Structure specifying a two-dimensional subregion

# C Specifications
Rectangles are used to describe a specified rectangular region of pixels
within an image or framebuffer.
Rectangles include both an offset and an extent of the same dimensionality,
as described above.
Two-dimensional rectangles are defined by the structure
```c
// Provided by VK_VERSION_1_0
typedef struct VkRect2D {
    VkOffset2D    offset;
    VkExtent2D    extent;
} VkRect2D;
```

# Members
- [`offset`] is a [`Offset2D`] specifying the rectangle offset.
- [`extent`] is a [`Extent2D`] specifying the rectangle extent.

# Related
- [`crate::vulkan1_0`]
- [`BindImageMemoryDeviceGroupInfo`]
- [`ClearRect`]
- [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]
- [`DeviceGroupRenderPassBeginInfo`]
- [`DisplayPresentInfoKHR`]
- [`Extent2D`]
- [`Offset2D`]
- [`PipelineDiscardRectangleStateCreateInfoEXT`]
- [`PipelineViewportExclusiveScissorStateCreateInfoNV`]
- [`PipelineViewportStateCreateInfo`]
- [`RenderPassBeginInfo`]
- [`RenderingInfo`]
- [`cmd_set_discard_rectangle_ext`]
- [`cmd_set_exclusive_scissor_nv`]
- [`cmd_set_scissor`]
- [`cmd_set_scissor_with_count`]
- [`cmd_set_scissor_with_count_ext`]
- [`get_physical_device_present_rectangles_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        