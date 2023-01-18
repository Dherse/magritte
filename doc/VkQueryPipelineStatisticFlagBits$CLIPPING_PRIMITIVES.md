[`CLIPPING_PRIMITIVES`] specifies that
queries managed by the pool will count the number of primitives output
by the [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping) stage of the
pipeline.
The counter’s value is incremented each time a primitive passes the
primitive clipping stage.
The actual number of primitives output by the primitive clipping stage
for a particular input primitive is implementation-dependent but  **must** 
satisfy the following conditions:
 - If at least one vertex of the input primitive lies inside the clipping volume, the counter is incremented by one or more.
 - Otherwise, the counter is incremented by zero or more.