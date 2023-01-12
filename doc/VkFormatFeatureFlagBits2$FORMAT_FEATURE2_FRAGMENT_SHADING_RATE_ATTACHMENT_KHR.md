[`FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`]
specifies that an image view  **can**  be used as a
[fragment shading rate
attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment).
An implementation  **must**  not set this feature for formats with numeric
type other than `*UINT`, or set it as a buffer feature.