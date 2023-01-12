[`max_compute_work_group_count`][3] is
the maximum number of local workgroups that  **can**  be dispatched by a
single dispatching command.
These three values represent the maximum number of local workgroups for
the X, Y, and Z dimensions, respectively.
The workgroup count parameters to the dispatching commands  **must**  be less
than or equal to the corresponding limit.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dispatch](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dispatch).