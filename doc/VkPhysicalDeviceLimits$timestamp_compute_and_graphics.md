[`timestamp_compute_and_graphics`]
specifies support for timestamps on all graphics and compute queues.
If this limit is set to `VK_TRUE`, all queues that advertise the
`VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT` in the
[`QueueFamilyProperties`]::`queueFlags` support
[`QueueFamilyProperties`]::`timestampValidBits` of at least 36.
See [Timestamp Queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-timestamps).