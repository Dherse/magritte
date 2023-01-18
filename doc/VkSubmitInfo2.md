[VkSubmitInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html) - Structure specifying a queue submit operation

# C Specifications
The [`SubmitInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkSubmitInfo2 {
    VkStructureType                     sType;
    const void*                         pNext;
    VkSubmitFlags                       flags;
    uint32_t                            waitSemaphoreInfoCount;
    const VkSemaphoreSubmitInfo*        pWaitSemaphoreInfos;
    uint32_t                            commandBufferInfoCount;
    const VkCommandBufferSubmitInfo*    pCommandBufferInfos;
    uint32_t                            signalSemaphoreInfoCount;
    const VkSemaphoreSubmitInfo*        pSignalSemaphoreInfos;
} VkSubmitInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkSubmitInfo2 VkSubmitInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`SubmitFlagBits`].
- [`wait_semaphore_info_count`] is the number of elements in [`wait_semaphore_infos`].
- [`wait_semaphore_infos`] is a pointer to an array of [`SemaphoreSubmitInfo`] structures defining [semaphore wait operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).
- [`command_buffer_info_count`] is the number of elements in [`command_buffer_infos`] and the number of command buffers to execute in the batch.
- [`command_buffer_infos`] is a pointer to an array of [`CommandBufferSubmitInfo`] structures describing command buffers to execute in the batch.
- [`signal_semaphore_info_count`] is the number of elements in [`signal_semaphore_infos`].
- [`signal_semaphore_infos`] is a pointer to an array of [`SemaphoreSubmitInfo`] describing [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).

# Description
## Valid Usage
-    If the same semaphore is used as the `semaphore` member of both an element of [`signal_semaphore_infos`] and [`wait_semaphore_infos`], and that semaphore is a timeline semaphore, the `value` member of the [`signal_semaphore_infos`] element  **must**  be greater than the `value` member of the [`wait_semaphore_infos`] element
-    If the `semaphore` member of any element of [`signal_semaphore_infos`] is a timeline semaphore, the `value` member of that element  **must**  have a value greater than the current value of the semaphore when the [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) is executed
-    If the `semaphore` member of any element of [`signal_semaphore_infos`] is a timeline semaphore, the `value` member of that element  **must**  have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
-    If the `semaphore` member of any element of [`wait_semaphore_infos`] is a timeline semaphore, the `value` member of that element  **must**  have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
-    If [`flags`] includes `VK_SUBMIT_PROTECTED_BIT`, all elements of `pCommandBuffers` **must**  be protected command buffers
-    If [`flags`] does not include `VK_SUBMIT_PROTECTED_BIT`, each element of `pCommandBuffers` **must**  not be a protected command buffer
-    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [resumed render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they  **must**  be suspended by a render pass instance earlier in submission order within [`command_buffer_infos`]
-    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they  **must**  be resumed by a render pass instance later in submission order within [`command_buffer_infos`]
-    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there  **must**  be no action or synchronization commands between that render pass instance and the render pass instance that resumes it
-    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there  **must**  be no render pass instances between that render pass instance and the render pass instance that resumes it
-    If the [`variableSampleLocations`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-variableSampleLocations) limit is not supported, and any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), where a graphics pipeline has been bound, any pipelines bound in the render pass instance that resumes it, or any subsequent render pass instances that resume from that one and so on,  **must**  use the same sample locations

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PerformanceQuerySubmitInfoKHR`], [`Win32KeyedMutexAcquireReleaseInfoKHR`], or [`Win32KeyedMutexAcquireReleaseInfoNV`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`SubmitFlagBits`] values
-    If [`wait_semaphore_info_count`] is not `0`, [`wait_semaphore_infos`] **must**  be a valid pointer to an array of [`wait_semaphore_info_count`] valid [`SemaphoreSubmitInfo`] structures
-    If [`command_buffer_info_count`] is not `0`, [`command_buffer_infos`] **must**  be a valid pointer to an array of [`command_buffer_info_count`] valid [`CommandBufferSubmitInfo`] structures
-    If [`signal_semaphore_info_count`] is not `0`, [`signal_semaphore_infos`] **must**  be a valid pointer to an array of [`signal_semaphore_info_count`] valid [`SemaphoreSubmitInfo`] structures

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBufferSubmitInfo`]
- [`SemaphoreSubmitInfo`]
- [`StructureType`]
- [`SubmitFlags`]
- [`queue_submit2`]
- [`queue_submit2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        