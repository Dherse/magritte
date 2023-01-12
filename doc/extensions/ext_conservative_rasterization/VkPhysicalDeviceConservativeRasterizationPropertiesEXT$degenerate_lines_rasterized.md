[`degenerate_lines_rasterized`] is
`VK_FALSE` if the implementation culls lines that become zero length
after they are quantized to the fixed-point rasterization pixel grid.
[`degenerate_lines_rasterized`] is `VK_TRUE` if zero length lines
are not culled and the provoking vertex attributes and depth value are
used for the fragments.