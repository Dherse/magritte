[VkOffset2D](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOffset2D.html) - Structure specifying a two-dimensional offset

# C Specifications
A two-dimensional offset is defined by the structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkOffset2D {
    int32_t    x;
    int32_t    y;
} VkOffset2D;
```

# Members
- [`x`] is the x offset.
- [`y`] is the y offset.

# Related
- [`crate::vulkan1_0`]
- [`DisplayPlaneCapabilitiesKHR`]
- [`Rect2D`]
- [`RectLayerKHR`]
- [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
- [`VideoDecodeH264CapabilitiesEXT`]
- [`VideoDecodeInfoKHR`]
- [`VideoPictureResourceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        