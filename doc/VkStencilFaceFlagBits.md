[VkStencilFaceFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html) - Bitmask specifying sets of stencil state for which to update the compare mask

# C Specifications
[`StencilFaceFlagBits`] values are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkStencilFaceFlagBits {
    VK_STENCIL_FACE_FRONT_BIT = 0x00000001,
    VK_STENCIL_FACE_BACK_BIT = 0x00000002,
    VK_STENCIL_FACE_FRONT_AND_BACK = 0x00000003,
    VK_STENCIL_FRONT_AND_BACK = VK_STENCIL_FACE_FRONT_AND_BACK,
} VkStencilFaceFlagBits;
```

# Description
- [`VK_STENCIL_FACE_FLAG_BITS`] specifies that only the front set of stencil state is updated.
- [`VK_STENCIL_FACE_FLAG_BITS`] specifies that only the back set of stencil state is updated.
- [`VK_STENCIL_FACE_FLAG_BITS`] is the combination of [`VK_STENCIL_FACE_FLAG_BITS`] and [`VK_STENCIL_FACE_FLAG_BITS`], and specifies that both sets of stencil state are updated.

# Related
- [`crate::vulkan1_0`]
- [VkStencilFaceFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        