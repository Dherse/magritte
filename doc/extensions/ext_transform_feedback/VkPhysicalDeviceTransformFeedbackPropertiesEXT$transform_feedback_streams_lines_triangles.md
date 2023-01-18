[`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the
implementation supports the geometry shader `OpExecutionMode` of
`OutputLineStrip` and `OutputTriangleStrip` in addition to
`OutputPoints` when more than one vertex stream is output.
If [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the
implementation only supports an `OpExecutionMode` of
`OutputPoints` when more than one vertex stream is output from the
geometry shader.