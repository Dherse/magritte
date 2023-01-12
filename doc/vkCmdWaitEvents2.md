[vkCmdWaitEvents2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWaitEvents2.html) - Wait for one or more events

# C Specifications
To wait for one or more events to enter the signaled state on a device,
call:
```c
// Provided by VK_VERSION_1_3
void vkCmdWaitEvents2(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    eventCount,
    const VkEvent*                              pEvents,
    const VkDependencyInfo*                     pDependencyInfos);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
void vkCmdWaitEvents2KHR(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    eventCount,
    const VkEvent*                              pEvents,
    const VkDependencyInfo*                     pDependencyInfos);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`event_count`] is the length of the [`p_events`] array.
- [`p_events`] is a pointer to an array of [`event_count`] events to wait on.
- [`p_dependency_infos`] is a pointer to an array of [`event_count`][`DependencyInfo`] structures, defining the second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).

# Description
When [`cmd_wait_events2`] is submitted to a queue, it inserts memory
dependencies according to the elements of [`p_dependency_infos`] and each
corresponding element of [`p_events`].
[`cmd_wait_events2`] **must**  not be used to wait on event signal operations
occurring on other queues, or signal operations executed by
[`cmd_set_event`].The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) of each memory
dependency defined by any element i of [`p_dependency_infos`] are
applied to operations that occurred earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) than the last event
signal operation on element i of [`p_events`].Signal operations for an event at index i are only included if:
- The event was signaled by a [`cmd_set_event2`] command that occurred earlier in [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) with a `dependencyInfo` parameter exactly equal to the element of [`p_dependency_infos`] at index i ; or
- The event was created without `VK_EVENT_CREATE_DEVICE_ONLY_BIT`, and the first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) defined by the element of [`p_dependency_infos`] at index i only includes host operations (`VK_PIPELINE_STAGE_2_HOST_BIT`).
The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) of each
memory dependency defined by any element i of [`p_dependency_infos`]
are applied to operations that occurred later in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order) than
[`cmd_wait_events2`].
## Valid Usage
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-    Members of [`p_events`] **must**  not have been signaled by [`cmd_set_event`]
-    For any element i of [`p_events`], if that event is signaled by [`cmd_set_event2`], that command’s `dependencyInfo` parameter  **must**  be exactly equal to the ith element of [`p_dependency_infos`]
-    For any element i of [`p_events`], if that event is signaled by [`set_event`], barriers in the ith element of [`p_dependency_infos`] **must**  include only host operations in their first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
-    For any element i of [`p_events`], if barriers in the ith element of [`p_dependency_infos`] include only host operations, the ith element of [`p_events`] **must**  be signaled before [`cmd_wait_events2`] is executed
-    For any element i of [`p_events`], if barriers in the ith element of [`p_dependency_infos`] do not include host operations, the ith element of [`p_events`] **must**  be signaled by a corresponding [`cmd_set_event2`] that occurred earlier in [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order)
-    The `srcStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_infos`] **must**  either include only pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from, or include only `VK_PIPELINE_STAGE_2_HOST_BIT`
-    The `dstStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_infos`] **must**  only include pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from
-    The `dependencyFlags` member of any element of `pDependencyInfo` **must**  be `0`
-    If [`p_events`] includes one or more events that will be signaled by [`set_event`] after [`command_buffer`] has been submitted to a queue, then [`cmd_wait_events2`] **must**  not be called inside a render pass instance
-  [`command_buffer`]’s current device mask  **must**  include exactly one physical device

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_events`] **must**  be a valid pointer to an array of [`event_count`] valid [`Event`] handles
-  [`p_dependency_infos`] **must**  be a valid pointer to an array of [`event_count`] valid [`DependencyInfo`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-  [`event_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and the elements of [`p_events`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`DependencyInfo`]
- [`Event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        