[VkDeviceGroupSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html) - Structure indicating which physical devices execute semaphore operations and command buffers

# C Specifications
If the [`p_next`] chain of [`SubmitInfo`] includes a
[`DeviceGroupSubmitInfo`] structure, then that structure includes device
indices and masks specifying which physical devices execute semaphore
operations and command buffers.The [`DeviceGroupSubmitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDeviceGroupSubmitInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           waitSemaphoreCount;
    const uint32_t*    pWaitSemaphoreDeviceIndices;
    uint32_t           commandBufferCount;
    const uint32_t*    pCommandBufferDeviceMasks;
    uint32_t           signalSemaphoreCount;
    const uint32_t*    pSignalSemaphoreDeviceIndices;
} VkDeviceGroupSubmitInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkDeviceGroupSubmitInfo VkDeviceGroupSubmitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_count`] is the number of elements in the [`wait_semaphore_device_indices`] array.
- [`wait_semaphore_device_indices`] is a pointer to an array of [`wait_semaphore_count`] device indices indicating which physical device executes the semaphore wait operation in the corresponding element of [`SubmitInfo::wait_semaphores`].
- [`command_buffer_count`] is the number of elements in the [`command_buffer_device_masks`] array.
- [`command_buffer_device_masks`] is a pointer to an array of [`command_buffer_count`] device masks indicating which physical devices execute the command buffer in the corresponding element of [`SubmitInfo::command_buffers`]. A physical device executes the command buffer if the corresponding bit is set in the mask.
- [`signal_semaphore_count`] is the number of elements in the [`signal_semaphore_device_indices`] array.
- [`signal_semaphore_device_indices`] is a pointer to an array of [`signal_semaphore_count`] device indices indicating which physical device executes the semaphore signal operation in the corresponding element of [`SubmitInfo::signal_semaphores`].

# Description
If this structure is not present, semaphore operations and command buffers
execute on device index zero.
## Valid Usage
-  [`wait_semaphore_count`] **must**  equal [`SubmitInfo`]::[`wait_semaphore_count`]
-  [`command_buffer_count`] **must**  equal [`SubmitInfo`]::[`command_buffer_count`]
-  [`signal_semaphore_count`] **must**  equal [`SubmitInfo`]::[`signal_semaphore_count`]
-    All elements of [`wait_semaphore_device_indices`] and [`signal_semaphore_device_indices`] **must**  be valid device indices
-    All elements of [`command_buffer_device_masks`] **must**  be valid device masks

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`
-    If [`wait_semaphore_count`] is not `0`, [`wait_semaphore_device_indices`] **must**  be a valid pointer to an array of [`wait_semaphore_count`]`uint32_t` values
-    If [`command_buffer_count`] is not `0`, [`command_buffer_device_masks`] **must**  be a valid pointer to an array of [`command_buffer_count`]`uint32_t` values
-    If [`signal_semaphore_count`] is not `0`, [`signal_semaphore_device_indices`] **must**  be a valid pointer to an array of [`signal_semaphore_count`]`uint32_t` values

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        