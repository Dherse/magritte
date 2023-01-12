[`max_vertex_input_bindings`] is the
maximum number of vertex buffers that  **can**  be specified for providing
vertex attributes to a graphics pipeline.
These are described in the array of
[`VertexInputBindingDescription`] structures that are provided at
graphics pipeline creation time via the `pVertexBindingDescriptions`
member of the [`PipelineVertexInputStateCreateInfo`] structure.
The `binding` member of [`VertexInputBindingDescription`] **must** 
be less than this limit.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input).