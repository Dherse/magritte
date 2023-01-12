[`wide_lines`] specifies whether lines with
width other than 1.0 are supported.
If this feature is not enabled, the `lineWidth` member of the
[`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to
1.0 unless the `VK_DYNAMIC_STATE_LINE_WIDTH` dynamic state is
enabled, and the `lineWidth` parameter to [`cmd_set_line_width`] **must**  be set to 1.0.
When this feature is supported, the range and granularity of supported
line widths are indicated by the `lineWidthRange` and
`lineWidthGranularity` members of the [`PhysicalDeviceLimits`]
structure, respectively.