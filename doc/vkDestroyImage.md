[vkDestroyImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyImage.html) - Destroy an image object

# C Specifications
To destroy an image, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyImage(
    VkDevice                                    device,
    VkImage                                     image,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the image.
- [`image`] is the image to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    All submitted commands that refer to [`image`], either directly or via a [`ImageView`],  **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`image`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`image`] was created, [`p_allocator`] **must**  be `NULL`
-  [`image`] **must**  not have been acquired from [`get_swapchain_images_khr`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`image`] is not [`crate::Handle::null`], [`image`] **must**  be a valid [`Image`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`image`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`image`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`Image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        