[VkFramebuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html) - Opaque handle to a framebuffer object

# C Specifications
Render passes operate in conjunction with *framebuffers*.
Framebuffers represent a collection of specific memory attachments that a
render pass instance uses.Framebuffers are represented by [`Framebuffer`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFramebuffer)
```

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferInheritanceInfo`]
- [`RenderPassBeginInfo`]
- [`create_framebuffer`]
- [`destroy_framebuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        