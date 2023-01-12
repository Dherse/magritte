[VkD3D12FenceSubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) - Structure specifying values for Direct3D 12 fence-backed semaphores

# C Specifications
To specify the values to use when waiting for and signaling semaphores whose
[current payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) refers to a
Direct3D 12 fence, add a [`D3d12FenceSubmitInfoKHR`] structure to the
[`p_next`] chain of the [`SubmitInfo`] structure.
The [`D3d12FenceSubmitInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_external_semaphore_win32
typedef struct VkD3D12FenceSubmitInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           waitSemaphoreValuesCount;
    const uint64_t*    pWaitSemaphoreValues;
    uint32_t           signalSemaphoreValuesCount;
    const uint64_t*    pSignalSemaphoreValues;
} VkD3D12FenceSubmitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_values_count`] is the number of semaphore wait values specified in [`wait_semaphore_values`].
- [`wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_values_count`] values for the corresponding semaphores in [`SubmitInfo::wait_semaphores`] to wait for.
- [`signal_semaphore_values_count`] is the number of semaphore signal values specified in [`signal_semaphore_values`].
- [`signal_semaphore_values`] is a pointer to an array of [`signal_semaphore_values_count`] values for the corresponding semaphores in [`SubmitInfo::signal_semaphores`] to set when signaled.

# Description
If the semaphore in [`SubmitInfo::wait_semaphores`] or
[`SubmitInfo::signal_semaphores`] corresponding to an entry in
[`wait_semaphore_values`] or [`signal_semaphore_values`] respectively does
not currently have a [payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-payloads)
referring to a Direct3D 12 fence, the implementation  **must**  ignore the value
in the [`wait_semaphore_values`] or [`signal_semaphore_values`] entry.
## Valid Usage
-  [`wait_semaphore_values_count`] **must**  be the same value as [`SubmitInfo::wait_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of this [`D3d12FenceSubmitInfoKHR`] structure
-  [`signal_semaphore_values_count`] **must**  be the same value as [`SubmitInfo::signal_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of this [`D3d12FenceSubmitInfoKHR`] structure

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
-    If [`wait_semaphore_values_count`] is not `0`, and [`wait_semaphore_values`] is not `NULL`, [`wait_semaphore_values`] **must**  be a valid pointer to an array of [`wait_semaphore_values_count`]`uint64_t` values
-    If [`signal_semaphore_values_count`] is not `0`, and [`signal_semaphore_values`] is not `NULL`, [`signal_semaphore_values`] **must**  be a valid pointer to an array of [`signal_semaphore_values_count`]`uint64_t` values

# Related
- [`khr_external_semaphore_win32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        