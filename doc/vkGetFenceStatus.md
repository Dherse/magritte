[vkGetFenceStatus](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceStatus.html) - Return the status of a fence

# C Specifications
To query the status of a fence from the host, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkGetFenceStatus(
    VkDevice                                    device,
    VkFence                                     fence);
```

# Parameters
- [`device`] is the logical device that owns the fence.
- [`fence`] is the handle of the fence to query.

# Description
Upon success, [`get_fence_status`] returns the status of the fence object,
with the following return codes:If a [queue submission](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission) command is pending
execution, then the value returned by this command  **may**  immediately be out
of date.If the device has been lost (see [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device)),
[`get_fence_status`] **may**  return any of the above status codes.
If the device has been lost and [`get_fence_status`] is called repeatedly,
it will eventually return either `VK_SUCCESS` or
`VK_ERROR_DEVICE_LOST`.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`fence`] **must**  be a valid [`Fence`] handle
-  [`fence`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_NOT_READY` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Fence`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        