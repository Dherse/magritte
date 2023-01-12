[`transform_feedback_preserves_provoking_vertex`] indicates that the order
of vertices within each primitive written by transform feedback will
preserve the provoking vertex.
This does not apply to triangle fan primitives when
[`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex)
is `VK_FALSE`.
[`transform_feedback_preserves_provoking_vertex`] **must**  be `VK_FALSE`
when the [`ext_transform_feedback`] extension is not supported.