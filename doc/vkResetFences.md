[vkResetFences](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetFences.html) - Resets one or more fence objects

# C Specifications
To set the state of fences to unsignaled from the host, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkResetFences(
    VkDevice                                    device,
    uint32_t                                    fenceCount,
    const VkFence*                              pFences);
```

# Parameters
- [`device`] is the logical device that owns the fences.
- [`fence_count`] is the number of fences to reset.
- [`p_fences`] is a pointer to an array of fence handles to reset.

# Description
If any member of [`p_fences`] currently has its
[payload imported](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) with temporary
permanence, that fence’s prior permanent payload is first restored.
The remaining operations described therefore operate on the restored
payload.When [`reset_fences`] is executed on the host, it defines a *fence
unsignal operation* for each fence, which resets the fence to the unsignaled
state.If any member of [`p_fences`] is already in the unsignaled state when
[`reset_fences`] is executed, then [`reset_fences`] has no effect on
that fence.
## Valid Usage
-    Each element of [`p_fences`] **must**  not be currently associated with any queue command that has not yet completed execution on that queue

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_fences`] **must**  be a valid pointer to an array of [`fence_count`] valid [`Fence`] handles
-  [`fence_count`] **must**  be greater than `0`
-    Each element of [`p_fences`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to each member of [`p_fences`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Fence`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        