[vkGetSwapchainCounterEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html) - Query the current value of a surface counter

# C Specifications
The requested counters become active when the first presentation command for
the associated swapchain is processed by the presentation engine.
To query the value of an active counter, use:
```c
// Provided by VK_EXT_display_control
VkResult vkGetSwapchainCounterEXT(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    VkSurfaceCounterFlagBitsEXT                 counter,
    uint64_t*                                   pCounterValue);
```

# Parameters
- [`device`] is the [`Device`] associated with [`swapchain`].
- [`swapchain`] is the swapchain from which to query the counter value.
- [`counter`] is a [`SurfaceCounterFlagBitsEXT`] value specifying the counter to query.
- [`p_counter_value`] will return the current value of the counter.

# Description
If a counter is not available because the swapchain is out of date, the
implementation  **may**  return `VK_ERROR_OUT_OF_DATE_KHR`.
## Valid Usage
-    One or more present commands on [`swapchain`] **must**  have been processed by the presentation engine

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-  [`counter`] **must**  be a valid [`SurfaceCounterFlagBitsEXT`] value
-  [`p_counter_value`] **must**  be a valid pointer to a `uint64_t` value
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`

# Related
- [`ext_display_control`]
- [`Device`]
- [`SurfaceCounterFlagBitsEXT`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        