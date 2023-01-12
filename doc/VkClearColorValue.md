[VkClearColorValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearColorValue.html) - Structure specifying a clear color value

# C Specifications
The [`ClearColorValue`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef union VkClearColorValue {
    float       float32[4];
    int32_t     int32[4];
    uint32_t    uint32[4];
} VkClearColorValue;
```

# Members
- [`float32`] are the color clear values when the format of the image or attachment is one of the formats in the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table other than signed integer (`SINT`) or unsigned integer (`UINT`). Floating point values are automatically converted to the format of the image, with the clear value being treated as linear if the image is sRGB.
- [`int32`] are the color clear values when the format of the image or attachment is signed integer (`SINT`). Signed integer values are converted to the format of the image by casting to the smaller type (with negative 32-bit values mapping to negative values in the smaller type). If the integer clear value is not representable in the target type (e.g. would overflow in conversion to that type), the clear value is undefined.
- [`uint32`] are the color clear values when the format of the image or attachment is unsigned integer (`UINT`). Unsigned integer values are converted to the format of the image by casting to the integer type with fewer bits.

# Description
The four array elements of the clear color map to R, G, B, and A components
of image formats, in order.If the image has more than one sample, the same value is written to all
samples for any pixels being cleared.

# Related
- [`crate::vulkan1_0`]
- [`ClearValue`]
- [`SamplerCustomBorderColorCreateInfoEXT`]
- [`cmd_clear_color_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        