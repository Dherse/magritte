[VkDisplayPlaneAlphaFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html) - Alpha blending type

# C Specifications
Bits which  **can**  be set in
[`DisplaySurfaceCreateInfoKHR::alpha_mode`], specifying the type of
alpha blending to use on a display, are:
```c
// Provided by VK_KHR_display
typedef enum VkDisplayPlaneAlphaFlagBitsKHR {
    VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
    VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
    VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008,
} VkDisplayPlaneAlphaFlagBitsKHR;
```

# Description
- [`VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_KHR`] specifies that the source image will be treated as opaque.
- [`VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_KHR`] specifies that a global alpha value  **must**  be specified that will be applied to all pixels in the source image.
- [`VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_KHR`] specifies that the alpha value will be determined by the alpha component of the source image’s pixels. If the source format contains no alpha values, no blending will be applied. The source alpha values are not premultiplied into the source image’s other color components.
- [`VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_KHR`] is equivalent to [`VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_KHR`], except the source alpha values are assumed to be premultiplied into the source image’s other color components.

# Related
- [`khr_display`]
- [VkDisplayPlaneAlphaFlagsKHR]()
- [`DisplaySurfaceCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        