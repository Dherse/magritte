[VkImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html) - Opaque handle to an image object

# C Specifications
Images represent multidimensional - up to 3 - arrays of data which  **can**  be
used for various purposes (e.g. attachments, textures), by binding them to a
graphics or compute pipeline via descriptor sets, or by directly specifying
them as parameters to certain commands.Images are represented by [`Image`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImage)
```

# Related
- [`crate::vulkan1_0`]
- [`BindImageMemoryInfo`]
- [`BlitImageInfo2`]
- [`CopyBufferToImageInfo2`]
- [`CopyImageInfo2`]
- [`CopyImageToBufferInfo2`]
- [`DedicatedAllocationMemoryAllocateInfoNV`]
- [`ImageMemoryBarrier`]
- [`ImageMemoryBarrier2`]
- [`ImageMemoryRequirementsInfo2`]
- [`ImageSparseMemoryRequirementsInfo2`]
- [`ImageViewCreateInfo`]
- [`MemoryDedicatedAllocateInfo`]
- [`ResolveImageInfo2`]
- [`SparseImageMemoryBindInfo`]
- [`SparseImageOpaqueMemoryBindInfo`]
- [`bind_image_memory`]
- [`cmd_blit_image`]
- [`cmd_clear_color_image`]
- [`cmd_clear_depth_stencil_image`]
- [`cmd_copy_buffer_to_image`]
- [`cmd_copy_image`]
- [`cmd_copy_image_to_buffer`]
- [`cmd_resolve_image`]
- [`create_image`]
- [`destroy_image`]
- [`get_image_drm_format_modifier_properties_ext`]
- [`get_image_memory_requirements`]
- [`get_image_sparse_memory_requirements`]
- [`get_image_subresource_layout`]
- [`get_swapchain_images_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        