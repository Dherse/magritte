[`LINE_STIPPLE_EXT`] specifies that the
`lineStippleFactor` and `lineStipplePattern` state in
[`PipelineRasterizationLineStateCreateInfoEXT`] will be ignored and
 **must**  be set dynamically with [`cmd_set_line_stipple_ext`] before any
draws are performed with a pipeline state with
[`PipelineRasterizationLineStateCreateInfoEXT`] member
`stippledLineEnable` set to `VK_TRUE`.