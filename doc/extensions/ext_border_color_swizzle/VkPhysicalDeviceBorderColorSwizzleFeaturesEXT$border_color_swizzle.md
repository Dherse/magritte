[`border_color_swizzle`] indicates that
defined values are returned by sampled image operations when used with a
sampler that uses a `VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK`,
`VK_BORDER_COLOR_INT_OPAQUE_BLACK`,
`VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`, or
`VK_BORDER_COLOR_INT_CUSTOM_EXT``borderColor` and an image view
that uses a non-[identity
component mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), when either [`border_color_swizzle_from_image`] is
enabled or the [`SamplerBorderColorComponentMappingCreateInfoEXT`]
is specified.