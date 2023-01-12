[vkCreateEvent](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateEvent.html) - Create a new event object

# C Specifications
To create an event, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateEvent(
    VkDevice                                    device,
    const VkEventCreateInfo*                    pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkEvent*                                    pEvent);
```

# Parameters
- [`device`] is the logical device that creates the event.
- [`p_create_info`] is a pointer to a [`EventCreateInfo`] structure containing information about how the event is to be created.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_event`] is a pointer to a handle in which the resulting event object is returned.

# Description
When created, the event object is in the unsignaled state.
## Valid Usage
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::events`] is `VK_FALSE`, then the implementation does not support [events](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events), and [`create_event`] **must**  not be used

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`EventCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_event`] **must**  be a valid pointer to a [`Event`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`Event`]
- [`EventCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        