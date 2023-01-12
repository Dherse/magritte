[vkGetDeferredOperationMaxConcurrencyKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) - Query the maximum concurrency on a deferred operation

# C Specifications
To query the number of additional threads that can usefully be joined to a
deferred operation, call:
```c
// Provided by VK_KHR_deferred_host_operations
uint32_t vkGetDeferredOperationMaxConcurrencyKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      operation);
```

# Parameters
- [`device`] is the device which owns [`operation`].
- [`operation`] is the deferred operation to be queried.

# Description
The returned value is the maximum number of threads that can usefully
execute a deferred operation concurrently, reported for the state of the
deferred operation at the point this command is called.
This value is intended to be used to better schedule work onto available
threads.
Applications  **can**  join any number of threads to the deferred operation and
expect it to eventually complete, though excessive joins  **may**  return
`VK_THREAD_DONE_KHR` immediately, performing no useful work.If [`operation`] is complete,
[`get_deferred_operation_max_concurrency_khr`] returns zero.If [`operation`] is currently joined to any threads, the value returned by
this command  **may**  immediately be out of date.If [`operation`] is pending, implementations  **must**  not return zero unless
at least one thread is currently executing [`deferred_operation_join_khr`]
on [`operation`].
If there are such threads, the implementation  **should**  return an estimate of
the number of additional threads which it could profitably use.Implementations  **may**  return 2<sup>32</sup>-1 to indicate that the maximum
concurrency is unknown and cannot be easily derived.
Implementations  **may**  return values larger than the maximum concurrency
available on the host CPU.
In these situations, an application  **should**  clamp the return value rather
than oversubscribing the machine.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`operation`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`khr_deferred_host_operations`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        