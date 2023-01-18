[vkDestroySurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html) - Destroy a VkSurfaceKHR object

# C Specifications
To destroy a [`SurfaceKHR`] object, call:
```c
// Provided by VK_KHR_surface
void vkDestroySurfaceKHR(
    VkInstance                                  instance,
    VkSurfaceKHR                                surface,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`instance`] is the instance used to create the surface.
- [`surface`] is the surface to destroy.
- [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).

# Description
Destroying a [`SurfaceKHR`] merely severs the connection between Vulkan
and the native surface, and does not imply destroying the native surface,
closing a window, or similar behavior.
## Valid Usage
-    All [`SwapchainKHR`] objects created for [`surface`] **must**  have been destroyed prior to destroying [`surface`]
-    If [`AllocationCallbacks`] were provided when [`surface`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`surface`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-    If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`surface`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`instance`]

## Host Synchronization
- Host access to [`surface`] **must**  be externally synchronized

# Related
- [`VK_KHR_surface`]
- [`AllocationCallbacks`]
- [`Instance`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        