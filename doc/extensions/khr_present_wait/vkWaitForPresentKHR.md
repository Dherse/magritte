[vkWaitForPresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html) - Wait for presentation

# C Specifications
When the [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait) feature is enabled, an
application  **can**  wait for an image to be presented to the user by first
specifying a presentId for the target presentation by adding a
[`PresentIdKHR`] structure to the `pNext` chain of the
[`PresentInfoKHR`] structure and then waiting for that presentation to
complete by calling:
```c
// Provided by VK_KHR_present_wait
VkResult vkWaitForPresentKHR(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    uint64_t                                    presentId,
    uint64_t                                    timeout);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the non-retired swapchain on which an image was queued for presentation.
- [`present_id`] is the presentation presentId to wait for.
- [`timeout`] is the timeout period in units of nanoseconds. [`timeout`] is adjusted to the closest value allowed by the implementation-dependent timeout accuracy, which  **may**  be substantially longer than one nanosecond, and  **may**  be longer than the requested period.

# Description
[`wait_for_present_khr`] waits for the presentId associated with
[`swapchain`] to be increased in value so that it is at least equal to
[`present_id`].For `VK_PRESENT_MODE_MAILBOX_KHR` (or other present mode where images
may be replaced in the presentation queue) any wait of this type associated
with such an image  **must**  be signaled no later than a wait associated with
the replacing image would be signaled.When the presentation has completed, the presentId associated with the
related `pSwapchains` entry will be increased in value so that it is at
least equal to the value provided in the [`PresentIdKHR`] structure.There is no requirement for any precise timing relationship between the
presentation of the image to the user and the update of the presentId value,
but implementations  **should**  make this as close as possible to the
presentation of the first pixel in the new image to the user.The call to [`wait_for_present_khr`] will block until either the presentId
associated with [`swapchain`] is greater than or equal to [`present_id`],
or [`timeout`] nanoseconds passes.
When the swapchain becomes OUT_OF_DATE, the call will either return
`VK_SUCCESS` (if the image was delivered to the presentation engine and
may have been presented to the user) or will return early with status
`VK_ERROR_OUT_OF_DATE_KHR` (if the image was not presented to the user).As an exception to the normal rules for objects which are externally
synchronized, the [`swapchain`] passed to [`wait_for_present_khr`] **may** 
be simultaneously used by other threads in calls to functions other than
[`destroy_swapchain_khr`].
Access to the swapchain data associated with this extension  **must**  be atomic
within the implementation.
## Valid Usage
-  [`swapchain`] **must**  not be in the retired state
-    The [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`  - `VK_TIMEOUT` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`khr_present_wait`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        