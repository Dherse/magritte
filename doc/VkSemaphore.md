[VkSemaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html) - Opaque handle to a semaphore object

# C Specifications
Semaphores are a synchronization primitive that  **can**  be used to insert a
dependency
between queue operations or between a queue operation and the host.
[Binary semaphores](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary) have two states - signaled and unsignaled.
[Timeline semaphores](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary) have a strictly increasing 64-bit unsigned
integer payload and are signaled with respect to a particular reference
value.
A semaphore  **can**  be signaled after execution of a queue operation is
completed, and a queue operation  **can**  wait for a semaphore to become
signaled before it begins execution.
A timeline semaphore  **can**  additionally be signaled from the host with the
[`signal_semaphore`] command and waited on from the host with the
[`wait_semaphores`] command.The internal data of a semaphore  **may**  include a reference to any resources
and pending work associated with signal or unsignal operations performed on
that semaphore object, collectively referred to as the semaphore’s
*payload*.
Mechanisms to import and export that internal data to and from semaphores
are provided [`ExportSemaphoreCreateInfo`].
These mechanisms indirectly enable applications to share semaphore state
between two or more semaphores and other synchronization primitives across
process and API boundaries.Semaphores are represented by [`Semaphore`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSemaphore)
```

# Related
- [`crate::vulkan1_0`]
- [`AcquireNextImageInfoKHR`]
- [`BindSparseInfo`]
- [`ImportSemaphoreFdInfoKHR`]
- [`ImportSemaphoreWin32HandleInfoKHR`]
- [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
- [`PresentInfoKHR`]
- [`SemaphoreGetFdInfoKHR`]
- [`SemaphoreGetWin32HandleInfoKHR`]
- [`SemaphoreGetZirconHandleInfoFUCHSIA`]
- [`SemaphoreSignalInfo`]
- [`SemaphoreSubmitInfo`]
- [`SemaphoreWaitInfo`]
- [`SubmitInfo`]
- [`acquire_next_image_khr`]
- [`create_semaphore`]
- [`destroy_semaphore`]
- [`get_semaphore_counter_value`]
- [`get_semaphore_counter_value_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        