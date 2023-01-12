[vkSetEvent](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetEvent.html) - Set an event to signaled state

# C Specifications
To set the state of an event to signaled from the host, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkSetEvent(
    VkDevice                                    device,
    VkEvent                                     event);
```

# Parameters
- [`device`] is the logical device that owns the event.
- [`event`] is the event to set.

# Description
When [`set_event`] is executed on the host, it defines an *event signal
operation* which sets the event to the signaled state.If [`event`] is already in the signaled state when [`set_event`] is
executed, then [`set_event`] has no effect, and no event signal operation
occurs.
## Valid Usage
-  [`event`] **must**  not have been created with `VK_EVENT_CREATE_DEVICE_ONLY_BIT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`event`] **must**  be a valid [`Event`] handle
-  [`event`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`event`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        