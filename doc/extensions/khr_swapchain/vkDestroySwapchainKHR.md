[vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html) - Destroy a swapchain object

# C Specifications
To destroy a swapchain object call:
```c
// Provided by VK_KHR_swapchain
void vkDestroySwapchainKHR(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the [`Device`] associated with [`swapchain`].
- [`swapchain`] is the swapchain to destroy.
- [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).

# Description
The application  **must**  not destroy a swapchain until after completion of all
outstanding operations on images that were acquired from the swapchain.
[`swapchain`] and all associated [`Image`] handles are destroyed, and
 **must**  not be acquired or used any more by the application.
The memory of each [`Image`] will only be freed after that image is no
longer used by the presentation engine.
For example, if one image of the swapchain is being displayed in a window,
the memory for that image  **may**  not be freed until the window is destroyed,
or another swapchain is created for the window.
Destroying the swapchain does not invalidate the parent [`SurfaceKHR`],
and a new swapchain  **can**  be created with it.When a swapchain associated with a display surface is destroyed, if the
image most recently presented to the display surface is from the swapchain
being destroyed, then either any display resources modified by presenting
images from any swapchain associated with the display surface  **must**  be
reverted by the implementation to their state prior to the first present
performed on one of these swapchains, or such resources  **must**  be left in
their current state.If [`swapchain`] has exclusive full-screen access, it is released before
the swapchain is destroyed.
## Valid Usage
-    All uses of presentable images acquired from [`swapchain`] **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`swapchain`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`swapchain`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`swapchain`] is not [`crate::Handle::null`], [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

# Related
- [`khr_swapchain`]
- [`AllocationCallbacks`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        