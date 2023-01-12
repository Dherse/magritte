[`DISCARD_RECTANGLE_EXT`] specifies that the
`pDiscardRectangles` state in
[`PipelineDiscardRectangleStateCreateInfoEXT`] will be ignored and
 **must**  be set dynamically with [`cmd_set_discard_rectangle_ext`] before
any draw or clear commands.
The [`DiscardRectangleModeEXT`] and the number of active discard
rectangles is still specified by the `discardRectangleMode` and
`discardRectangleCount` members of
[`PipelineDiscardRectangleStateCreateInfoEXT`].