[`stippled_rectangular_lines`]
indicates whether the implementation supports
[stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with
`VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT` lines, or with
`VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT` lines if
[`PhysicalDeviceLimits`]::`strictLines` is [`TRUE`].