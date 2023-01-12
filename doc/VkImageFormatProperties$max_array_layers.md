[`max_array_layers`] is the maximum number of array layers.
[`max_array_layers`] **must**  be no less than
[`PhysicalDeviceLimits`]::`maxImageArrayLayers`, except when one
of the following conditions is true, in which case it  **may**  instead be
`1`:
 - `tiling` is `VK_IMAGE_TILING_LINEAR`
 - `tiling` is `VK_IMAGE_TILING_OPTIMAL` and `type` is `VK_IMAGE_TYPE_3D`
 - `format` is one of the [formats that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)