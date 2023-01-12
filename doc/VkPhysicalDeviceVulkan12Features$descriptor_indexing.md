[`descriptor_indexing`] indicates
whether the implementation supports the minimum set of descriptor
indexing features as described in the [Feature
Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section.
Enabling the [`descriptor_indexing`] member when [`create_device`]
is called does not imply the other minimum descriptor indexing features
are also enabled.
Those other descriptor indexing features  **must**  be enabled individually
as needed by the application.