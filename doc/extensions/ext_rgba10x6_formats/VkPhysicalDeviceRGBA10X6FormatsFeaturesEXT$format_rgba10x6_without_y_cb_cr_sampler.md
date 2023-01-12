[`format_rgba10x6_without_y_cb_cr_sampler`] indicates that
`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` **can**  be used with a
[`ImageView`] with `subresourceRange.aspectMask` equal to
`VK_IMAGE_ASPECT_COLOR_BIT` without a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.