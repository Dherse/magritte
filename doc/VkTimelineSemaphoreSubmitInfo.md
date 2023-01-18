[VkTimelineSemaphoreSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html) - Structure specifying signal and wait values for timeline semaphores

# C Specifications
To specify the values to use when waiting for and signaling semaphores
created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`,
add a [`TimelineSemaphoreSubmitInfo`] structure to the [`p_next`] chain
of the [`SubmitInfo`] structure when using [`queue_submit`] or the
[`BindSparseInfo`] structure when using [`queue_bind_sparse`].
The [`TimelineSemaphoreSubmitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkTimelineSemaphoreSubmitInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           waitSemaphoreValueCount;
    const uint64_t*    pWaitSemaphoreValues;
    uint32_t           signalSemaphoreValueCount;
    const uint64_t*    pSignalSemaphoreValues;
} VkTimelineSemaphoreSubmitInfo;
```
or the equivalent
```c
// Provided by VK_KHR_timeline_semaphore
typedef VkTimelineSemaphoreSubmitInfo VkTimelineSemaphoreSubmitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_value_count`] is the number of semaphore wait values specified in [`wait_semaphore_values`].
- [`wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_value_count`] values for the corresponding semaphores in [`SubmitInfo::wait_semaphores`] to wait for.
- [`signal_semaphore_value_count`] is the number of semaphore signal values specified in [`signal_semaphore_values`].
- [`signal_semaphore_values`] is a pointer to an array [`signal_semaphore_value_count`] values for the corresponding semaphores in [`SubmitInfo::signal_semaphores`] to set when signaled.

# Description
If the semaphore in [`SubmitInfo::wait_semaphores`] or
[`SubmitInfo::signal_semaphores`] corresponding to an entry in
[`wait_semaphore_values`] or [`signal_semaphore_values`] respectively was
not created with a [`SemaphoreType`] of
`VK_SEMAPHORE_TYPE_TIMELINE`, the implementation  **must**  ignore the value
in the [`wait_semaphore_values`] or [`signal_semaphore_values`] entry.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`
-    If [`wait_semaphore_value_count`] is not `0`, and [`wait_semaphore_values`] is not `NULL`, [`wait_semaphore_values`] **must**  be a valid pointer to an array of [`wait_semaphore_value_count`]`uint64_t` values
-    If [`signal_semaphore_value_count`] is not `0`, and [`signal_semaphore_values`] is not `NULL`, [`signal_semaphore_values`] **must**  be a valid pointer to an array of [`signal_semaphore_value_count`]`uint64_t` values

# Related
- [`VK_KHR_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        