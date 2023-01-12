[`float32`] are the color clear values when the format of the image or
attachment is one of the formats in the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table
other than signed integer (`SINT`) or unsigned integer (`UINT`).
Floating point values are automatically converted to the format of the
image, with the clear value being treated as linear if the image is
sRGB.