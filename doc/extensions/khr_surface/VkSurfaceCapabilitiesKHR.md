[VkSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html) - Structure describing capabilities of a surface

# C Specifications
The [`SurfaceCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_surface
typedef struct VkSurfaceCapabilitiesKHR {
    uint32_t                         minImageCount;
    uint32_t                         maxImageCount;
    VkExtent2D                       currentExtent;
    VkExtent2D                       minImageExtent;
    VkExtent2D                       maxImageExtent;
    uint32_t                         maxImageArrayLayers;
    VkSurfaceTransformFlagsKHR       supportedTransforms;
    VkSurfaceTransformFlagBitsKHR    currentTransform;
    VkCompositeAlphaFlagsKHR         supportedCompositeAlpha;
    VkImageUsageFlags                supportedUsageFlags;
} VkSurfaceCapabilitiesKHR;
```

# Description
- [`min_image_count`] is the minimum number of images the specified device supports for a swapchain created for the surface, and will be at least one.
- [`max_image_count`] is the maximum number of images the specified device supports for a swapchain created for the surface, and will be either 0, or greater than or equal to [`min_image_count`]. A value of 0 means that there is no limit on the number of images, though there  **may**  be limits related to the total amount of memory used by presentable images.
- [`current_extent`] is the current width and height of the surface, or the special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will be determined by the extent of a swapchain targeting the surface.
- [`min_image_extent`] contains the smallest valid swapchain extent for the surface on the specified device. The `width` and `height` of the extent will each be less than or equal to the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has the special value described above.
- [`max_image_extent`] contains the largest valid swapchain extent for the surface on the specified device. The `width` and `height` of the extent will each be greater than or equal to the corresponding `width` and `height` of [`min_image_extent`]. The `width` and `height` of the extent will each be greater than or equal to the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has the special value described above.
- [`max_image_array_layers`] is the maximum number of layers presentable images  **can**  have for a swapchain created for this device and surface, and will be at least one.
- [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] indicating the presentation transforms supported for the surface on the specified device. At least one bit will be set.
- [`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value indicating the surface’s current transform relative to the presentation engine’s natural orientation.
- [`supported_composite_alpha`] is a bitmask of [`CompositeAlphaFlagBitsKHR`], representing the alpha compositing modes supported by the presentation engine for the surface on the specified device, and at least one bit will be set. Opaque composition  **can**  be achieved in any alpha compositing mode by either using an image format that has no alpha component, or by ensuring that all pixels in the presentable images have an alpha value of 1.0.
- [`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing the ways the application  **can**  use the presentable images of a swapchain created with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set. Implementations  **may**  support additional usages.

# Related
- [`VK_KHR_surface`]
- [`CompositeAlphaFlagsKHR`]
- [`Extent2D`]
- [`ImageUsageFlags`]
- [`SurfaceCapabilities2KHR`]
- [`SurfaceTransformFlagBitsKHR`]
- [`SurfaceTransformFlagsKHR`]
- [`get_physical_device_surface_capabilities_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        