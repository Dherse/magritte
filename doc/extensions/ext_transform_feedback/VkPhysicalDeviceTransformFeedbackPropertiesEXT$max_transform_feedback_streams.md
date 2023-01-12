[`max_transform_feedback_streams`]
is the maximum number of vertex streams that can be output from geometry
shaders declared with the `GeometryStreams` capability.
If the implementation does not support
[`PhysicalDeviceTransformFeedbackFeaturesEXT`]::`geometryStreams`
then [`max_transform_feedback_streams`] **must**  be set to `1`.