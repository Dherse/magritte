[VkDeviceGroupRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) - Set the initial device mask and render areas for a render pass instance

# C Specifications
If the [`p_next`] chain of [`RenderPassBeginInfo`]
or [`RenderingInfo`]
includes a [`DeviceGroupRenderPassBeginInfo`] structure, then that
structure includes a device mask and set of render areas for the render pass
instance.The [`DeviceGroupRenderPassBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceGroupRenderPassBeginInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           deviceMask;
    uint32_t           deviceRenderAreaCount;
    const VkRect2D*    pDeviceRenderAreas;
} VkDeviceGroupRenderPassBeginInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkDeviceGroupRenderPassBeginInfo VkDeviceGroupRenderPassBeginInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`device_mask`] is the device mask for the render pass instance.
- [`device_render_area_count`] is the number of elements in the [`device_render_areas`] array.
- [`device_render_areas`] is a pointer to an array of [`Rect2D`] structures defining the render area for each physical device.

# Description
The [`device_mask`] serves several purposes.
It is an upper bound on the set of physical devices that  **can**  be used during
the render pass instance, and the initial device mask when the render pass
instance begins.
In addition, commands transitioning to the next subpass in a render pass
instance and commands ending the render pass instance, and, accordingly
render pass attachment load, store, and resolve operations and subpass
dependencies corresponding to the render pass instance, are executed on the
physical devices included in the device mask provided here.If [`device_render_area_count`] is not zero, then the elements of
[`device_render_areas`] override the value of
[`RenderPassBeginInfo::render_area`], and provide a render area
specific to each physical device.
These render areas serve the same purpose as
[`RenderPassBeginInfo::render_area`], including controlling the
region of attachments that are cleared by `VK_ATTACHMENT_LOAD_OP_CLEAR`
and that are resolved into resolve attachments.If this structure is not present, the render pass instance’s device mask is
the value of [`DeviceGroupCommandBufferBeginInfo`]::[`device_mask`].
If this structure is not present or if [`device_render_area_count`] is zero,
[`RenderPassBeginInfo::render_area`] is used for all physical
devices.
## Valid Usage
-  [`device_mask`] **must**  be a valid device mask value
-  [`device_mask`] **must**  not be zero
-  [`device_mask`] **must**  be a subset of the command buffer’s initial device mask
-  [`device_render_area_count`] **must**  either be zero or equal to the number of physical devices in the logical device
-    The `offset.x` member of any element of [`device_render_areas`] **must**  be greater than or equal to 0
-    The `offset.y` member of any element of [`device_render_areas`] **must**  be greater than or equal to 0
-    The sum of the `offset.x` and `extent.width` members of any element of [`device_render_areas`] **must**  be less than or equal to [`maxFramebufferWidth`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferWidth)
-    The sum of the `offset.y` and `extent.height` members of any element of [`device_render_areas`] **must**  be less than or equal to [`maxFramebufferHeight`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferHeight)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`
-    If [`device_render_area_count`] is not `0`, [`device_render_areas`] **must**  be a valid pointer to an array of [`device_render_area_count`][`Rect2D`] structures

# Related
- [`crate::vulkan1_1`]
- [`Rect2D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        