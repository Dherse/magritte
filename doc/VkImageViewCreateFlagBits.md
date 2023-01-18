[VkImageViewCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html) - Bitmask specifying additional parameters of an image view

# C Specifications
Bits which  **can**  be set in [`ImageViewCreateInfo::flags`],
specifying additional parameters of an image view, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageViewCreateFlagBits {
  // Provided by VK_EXT_fragment_density_map
    VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 0x00000001,
  // Provided by VK_EXT_fragment_density_map2
    VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT = 0x00000002,
} VkImageViewCreateFlagBits;
```

# Description
- [`FRAGMENT_DENSITY_MAP_DYNAMIC_EXT`] specifies that the fragment density map will be read by device during `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
- [`FRAGMENT_DENSITY_MAP_DEFERRED_EXT`] specifies that the fragment density map will be read by the host during [`end_command_buffer`] for the primary command buffer that the render pass is recorded into

# Related
- [`crate::vulkan1_0`]
- [`ImageViewCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        