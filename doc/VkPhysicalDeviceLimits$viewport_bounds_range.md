[`viewport_bounds_range`][2] is the
[minimum, maximum] range that the corners of a viewport  **must**  be
contained in.
This range  **must**  be at least [-2 × `size`, 2 ×
`size` - 1], where `size` =
max([`max_viewport_dimensions`][0], [`max_viewport_dimensions`][1]).
See [Controlling the Viewport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport).