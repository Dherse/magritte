[VkResolveModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) - Bitmask indicating supported depth and stencil resolve modes

# C Specifications
Possible values of
[`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] and
`stencilResolveMode`, specifying the depth and stencil resolve modes,
are:
```c
// Provided by VK_VERSION_1_2
typedef enum VkResolveModeFlagBits {
    VK_RESOLVE_MODE_NONE = 0,
    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = 0x00000001,
    VK_RESOLVE_MODE_AVERAGE_BIT = 0x00000002,
    VK_RESOLVE_MODE_MIN_BIT = 0x00000004,
    VK_RESOLVE_MODE_MAX_BIT = 0x00000008,
  // Provided by VK_KHR_depth_stencil_resolve
    VK_RESOLVE_MODE_NONE_KHR = VK_RESOLVE_MODE_NONE,
  // Provided by VK_KHR_depth_stencil_resolve
    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR = VK_RESOLVE_MODE_SAMPLE_ZERO_BIT,
  // Provided by VK_KHR_depth_stencil_resolve
    VK_RESOLVE_MODE_AVERAGE_BIT_KHR = VK_RESOLVE_MODE_AVERAGE_BIT,
  // Provided by VK_KHR_depth_stencil_resolve
    VK_RESOLVE_MODE_MIN_BIT_KHR = VK_RESOLVE_MODE_MIN_BIT,
  // Provided by VK_KHR_depth_stencil_resolve
    VK_RESOLVE_MODE_MAX_BIT_KHR = VK_RESOLVE_MODE_MAX_BIT,
} VkResolveModeFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_depth_stencil_resolve
typedef VkResolveModeFlagBits VkResolveModeFlagBitsKHR;
```

# Description
- [`VK_RESOLVE_MODE_FLAG_BITS`] indicates that no resolve operation is done.
- [`VK_RESOLVE_MODE_FLAG_BITS`] indicates that result of the resolve operation is equal to the value of sample 0.
- [`VK_RESOLVE_MODE_FLAG_BITS`] indicates that result of the resolve operation is the average of the sample values.
- [`VK_RESOLVE_MODE_FLAG_BITS`] indicates that result of the resolve operation is the minimum of the sample values.
- [`VK_RESOLVE_MODE_FLAG_BITS`] indicates that result of the resolve operation is the maximum of the sample values.

# Related
- [`khr_depth_stencil_resolve`]
- [`crate::vulkan1_2`]
- [`RenderingAttachmentInfo`]
- [VkResolveModeFlags]()
- [`SubpassDescriptionDepthStencilResolve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        