[VkCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferBeginInfo.html) - Structure specifying a command buffer begin operation

# C Specifications
The [`CommandBufferBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkCommandBufferBeginInfo {
    VkStructureType                          sType;
    const void*                              pNext;
    VkCommandBufferUsageFlags                flags;
    const VkCommandBufferInheritanceInfo*    pInheritanceInfo;
} VkCommandBufferBeginInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`CommandBufferUsageFlagBits`] specifying usage behavior for the command buffer.
- [`inheritance_info`] is a pointer to a [`CommandBufferInheritanceInfo`] structure, used if `commandBuffer` is a secondary command buffer. If this is a primary command buffer, then this value is ignored.

# Description
## Valid Usage
-    If [`flags`] contains `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`, the `framebuffer` member of [`inheritance_info`] **must**  be either [`crate::Handle::null`], or a valid [`Framebuffer`] that is compatible with the `renderPass` member of [`inheritance_info`]
-    If [`flags`] contains `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` and the `renderPass` member of [`inheritance_info`] is not [`crate::Handle::null`], `renderPass` **must**  be a valid [`RenderPass`]
-    If [`flags`] contains `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` and the `renderPass` member of [`inheritance_info`] is not [`crate::Handle::null`], the `subpass` member of [`inheritance_info`] **must**  be a valid subpass index within the `renderPass` member of [`inheritance_info`]
-    If [`flags`] contains `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` and the `renderPass` member of [`inheritance_info`] is [`crate::Handle::null`], the [`p_next`] chain of [`inheritance_info`] **must**  include a [`CommandBufferInheritanceRenderingInfo`] structure
-    If [`flags`] contains `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`, the `renderPass` member of [`inheritance_info`] is [`crate::Handle::null`], and the [`p_next`] chain of [`inheritance_info`] includes a [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] structure, the `colorAttachmentCount` member of that structure  **must**  be equal to the value of [`CommandBufferInheritanceRenderingInfo::color_attachment_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`DeviceGroupCommandBufferBeginInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`CommandBufferUsageFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferInheritanceInfo`]
- [`CommandBufferUsageFlags`]
- [`StructureType`]
- [`begin_command_buffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        