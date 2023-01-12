[`transform_feedback_rasterization_stream_select`] is `VK_TRUE` if the
implementation supports the `GeometryStreams` SPIR-V capability and
the application can use
[`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which
vertex stream output is used for rasterization.
Otherwise vertex stream `0` **must**  always be used for rasterization.