[`discrete_queue_priorities`] is the
number of discrete priorities that  **can**  be assigned to a queue based on
the value of each member of
[`DeviceQueueCreateInfo`]::`pQueuePriorities`.
This  **must**  be at least 2, and levels  **must**  be spread evenly over the
range, with at least one level at 1.0, and another at 0.0.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-priority).