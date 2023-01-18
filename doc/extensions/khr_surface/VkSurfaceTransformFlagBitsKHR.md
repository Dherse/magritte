[VkSurfaceTransformFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html) - Presentation transforms supported on a device

# C Specifications
Bits which  **may**  be set in
[`SurfaceCapabilitiesKHR::supported_transforms`] indicating the
presentation transforms supported for the surface on the specified device,
and possible values of
[`SurfaceCapabilitiesKHR::current_transform`] indicating the
surface’s current transform relative to the presentation engine’s natural
orientation, are:
```c
// Provided by VK_KHR_surface
typedef enum VkSurfaceTransformFlagBitsKHR {
    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
} VkSurfaceTransformFlagBitsKHR;
```

# Description
- [`IDENTITY`] specifies that image content is presented without being transformed.
- [`ROTATE90`] specifies that image content is rotated 90 degrees clockwise.
- [`ROTATE180`] specifies that image content is rotated 180 degrees clockwise.
- [`ROTATE270`] specifies that image content is rotated 270 degrees clockwise.
- [`HORIZONTAL_MIRROR`] specifies that image content is mirrored horizontally.
- [`HORIZONTAL_MIRROR_ROTATE90`] specifies that image content is mirrored horizontally, then rotated 90 degrees clockwise.
- [`HORIZONTAL_MIRROR_ROTATE180`] specifies that image content is mirrored horizontally, then rotated 180 degrees clockwise.
- [`HORIZONTAL_MIRROR_ROTATE270`] specifies that image content is mirrored horizontally, then rotated 270 degrees clockwise.
- [`INHERIT`] specifies that the presentation transform is not specified, and is instead determined by platform-specific considerations and mechanisms outside Vulkan.

# Related
- [`VK_KHR_surface`]
- [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]
- [`CopyCommandTransformInfoQCOM`]
- [`DisplaySurfaceCreateInfoKHR`]
- [`RenderPassTransformBeginInfoQCOM`]
- [`SurfaceCapabilities2EXT`]
- [`SurfaceCapabilitiesKHR`]
- [`SurfaceTransformFlagsKHR`]
- [`SwapchainCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        