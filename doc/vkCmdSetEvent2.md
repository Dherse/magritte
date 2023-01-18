[vkCmdSetEvent2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetEvent2.html) - Set an event object to signaled state

# C Specifications
To signal an event from a device, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetEvent2(
    VkCommandBuffer                             commandBuffer,
    VkEvent                                     event,
    const VkDependencyInfo*                     pDependencyInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
void vkCmdSetEvent2KHR(
    VkCommandBuffer                             commandBuffer,
    VkEvent                                     event,
    const VkDependencyInfo*                     pDependencyInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`event`] is the event that will be signaled.
- [`p_dependency_info`] is a pointer to a [`DependencyInfo`] structure defining the first scopes of this operation.

# Description
When [`cmd_set_event2`] is submitted to a queue, it defines the first half
of memory dependencies defined by [`p_dependency_info`], as well as an event
signal operation which sets the event to the signaled state.
A memory dependency is defined between the event signal operation and
commands that occur earlier in submission order.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) are defined by
the union of all the memory dependencies defined by [`p_dependency_info`],
and are applied to all operations that occur earlier in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
[Queue family ownership transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and
[image layout transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions)
defined by [`p_dependency_info`] are also included in the first scopes.The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
includes only the event signal operation, and any
[queue family ownership transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and
[image layout transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions)
defined by [`p_dependency_info`].The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes)
includes only [queue family ownership
transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and [image layout
transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).Future [`cmd_wait_events2`] commands rely on all values of each element in
[`p_dependency_info`] matching exactly with those used to signal the
corresponding event.
[`cmd_wait_events`] **must**  not be used to wait on the result of a signal
operation defined by [`cmd_set_event2`].If [`event`] is already in the signaled state when [`cmd_set_event2`] is
executed on the device, then [`cmd_set_event2`] has no effect, no event
signal operation occurs, and no dependency is generated.
## Valid Usage
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-    The `dependencyFlags` member of [`p_dependency_info`] **must**  be `0`
-    The current device mask of [`command_buffer`] **must**  include exactly one physical device
-    The `srcStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_info`] **must**  only include pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from
-    The `dstStageMask` member of any element of the `pMemoryBarriers`, `pBufferMemoryBarriers`, or `pImageMemoryBarriers` members of [`p_dependency_info`] **must**  only include pipeline stages valid for the queue family that was used to create the command pool that [`command_buffer`] was allocated from

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`event`] **must**  be a valid [`Event`] handle
-  [`p_dependency_info`] **must**  be a valid pointer to a valid [`DependencyInfo`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Both of [`command_buffer`], and [`event`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`DependencyInfo`]
- [`Event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        