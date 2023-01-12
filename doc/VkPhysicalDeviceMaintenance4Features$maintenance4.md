[`maintenance4`] indicates
that the implementation supports the following:
 - The application  **may**  destroy a [`PipelineLayout`] object immediately after using it to create another object.
 - `LocalSizeId` **can**  be used as an alternative to `LocalSize` to specify the local workgroup size with specialization constants.
 - Images created with identical creation parameters will always have the same alignment requirements.
 - The size memory requirement of a buffer or image is never greater than that of another buffer or image created with a greater or equal size.
 - Push constants do not have to be initialized before they are dynamically accessed.
 - The interface matching rules allow a larger output vector to match with a smaller input vector, with additional values being discarded.