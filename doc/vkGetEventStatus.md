[vkGetEventStatus](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetEventStatus.html) - Retrieve the status of an event object

# C Specifications
To query the state of an event from the host, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkGetEventStatus(
    VkDevice                                    device,
    VkEvent                                     event);
```

# Parameters
- [`device`] is the logical device that owns the event.
- [`event`] is the handle of the event to query.

# Description
Upon success, [`get_event_status`] returns the state of the event object
with the following return codes:If a [`cmd_set_event`] or [`cmd_reset_event`] command is in a command
buffer that is in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), then the
value returned by this command  **may**  immediately be out of date.The state of an event  **can**  be updated by the host.
The state of the event is immediately changed, and subsequent calls to
[`get_event_status`] will return the new state.
If an event is already in the requested state, then updating it to the same
state has no effect.
## Valid Usage
-  [`event`] **must**  not have been created with `VK_EVENT_CREATE_DEVICE_ONLY_BIT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`event`] **must**  be a valid [`Event`] handle
-  [`event`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_EVENT_SET`  - `VK_EVENT_RESET` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Event`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        