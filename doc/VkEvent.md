[VkEvent](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEvent.html) - Opaque handle to an event object

# C Specifications
Events are a synchronization primitive that  **can**  be used to insert a
fine-grained dependency between commands submitted to the same queue, or
between the host and a queue.
Events  **must**  not be used to insert a dependency between commands submitted
to different queues.
Events have two states - signaled and unsignaled.
An application  **can**  signal or unsignal an event either on the host or on the
device.
A device  **can**  be made to wait for an event to become signaled before
executing further operations.
No command exists to wait for an event to become signaled on the host, but
the current state of an event  **can**  be queried.Events are represented by [`Event`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkEvent)
```

# Related
- [`crate::vulkan1_0`]
- [`cmd_reset_event`]
- [`cmd_reset_event2`]
- [`cmd_reset_event2_khr`]
- [`cmd_set_event`]
- [`cmd_set_event2`]
- [`cmd_set_event2_khr`]
- [`cmd_wait_events`]
- [`cmd_wait_events2`]
- [`cmd_wait_events2_khr`]
- [`create_event`]
- [`destroy_event`]
- [`get_event_status`]
- [`reset_event`]
- [`set_event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        