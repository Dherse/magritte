[VkViewportSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html) - Structure specifying a viewport swizzle

# C Specifications
The [`ViewportSwizzleNV`] structure is defined as:
```c
// Provided by VK_NV_viewport_swizzle
typedef struct VkViewportSwizzleNV {
    VkViewportCoordinateSwizzleNV    x;
    VkViewportCoordinateSwizzleNV    y;
    VkViewportCoordinateSwizzleNV    z;
    VkViewportCoordinateSwizzleNV    w;
} VkViewportSwizzleNV;
```

# Members
- [`x`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to the x component of the primitive
- [`y`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to the y component of the primitive
- [`z`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to the z component of the primitive
- [`w`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to the w component of the primitive

# Description
## Valid Usage (Implicit)
-  [`x`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
-  [`y`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
-  [`z`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
-  [`w`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value

# Related
- [`VK_NV_viewport_swizzle`]
- [`PipelineViewportSwizzleStateCreateInfoNV`]
- [`ViewportCoordinateSwizzleNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        