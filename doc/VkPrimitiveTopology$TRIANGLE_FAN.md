[`TRIANGLE_FAN`] specifies a series of
[connected triangle primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans) with all
triangles sharing a common vertex.
If the `[`VK_KHR_portability_subset`]` extension is enabled, and
[`PhysicalDevicePortabilitySubsetFeaturesKHR`]::`triangleFans`
is [`FALSE`], then triangle fans are not supported by the
implementation, and [`TRIANGLE_FAN`] **must**  not
be used.