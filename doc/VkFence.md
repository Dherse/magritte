[VkFence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html) - Opaque handle to a fence object

# C Specifications
Fences are a synchronization primitive that  **can**  be used to insert a
dependency from a queue to the host.
Fences have two states - signaled and unsignaled.
A fence  **can**  be signaled as part of the execution of a
[queue submission](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission) command.
Fences  **can**  be unsignaled on the host with [`reset_fences`].
Fences  **can**  be waited on by the host with the [`wait_for_fences`] command,
and the current state  **can**  be queried with [`get_fence_status`].The internal data of a fence  **may**  include a reference to any resources and
pending work associated with signal or unsignal operations performed on that
fence object, collectively referred to as the fence’s *payload*.
Mechanisms to import and export that internal data to and from fences are
provided [`ExportFenceCreateInfo`].
These mechanisms indirectly enable applications to share fence state between
two or more fences and other synchronization primitives across process and
API boundaries.Fences are represented by [`Fence`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFence)
```

# Related
- [`crate::vulkan1_0`]
- [`AcquireNextImageInfoKHR`]
- [`FenceGetFdInfoKHR`]
- [`FenceGetWin32HandleInfoKHR`]
- [`ImportFenceFdInfoKHR`]
- [`ImportFenceWin32HandleInfoKHR`]
- [`acquire_next_image_khr`]
- [`create_fence`]
- [`destroy_fence`]
- [`get_fence_status`]
- [`queue_bind_sparse`]
- [`queue_submit`]
- [`queue_submit2`]
- [`queue_submit2_khr`]
- [`register_device_event_ext`]
- [`register_display_event_ext`]
- [`reset_fences`]
- [`wait_for_fences`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        