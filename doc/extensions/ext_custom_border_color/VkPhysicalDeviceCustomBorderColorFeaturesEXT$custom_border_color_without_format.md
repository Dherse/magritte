[`custom_border_color_without_format`] indicates that explicit formats are
not required for custom border colors and the value of the `format`
member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure
 **may**  be `VK_FORMAT_UNDEFINED`.
If this feature bit is not set, applications  **must**  provide the
[`Format`] of the image view(s) being sampled by this sampler in the
`format` member of the [`SamplerCustomBorderColorCreateInfoEXT`]
structure.