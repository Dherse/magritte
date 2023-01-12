[vkDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) - Destroy a debug messenger object

# C Specifications
To destroy a [`DebugUtilsMessengerEXT`] object, call:
```c
// Provided by VK_EXT_debug_utils
void vkDestroyDebugUtilsMessengerEXT(
    VkInstance                                  instance,
    VkDebugUtilsMessengerEXT                    messenger,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`instance`] is the instance where the callback was created.
- [`messenger`] is the [`DebugUtilsMessengerEXT`] object to destroy. [`messenger`] is an externally synchronized object and  **must**  not be used on more than one thread at a time. This means that [`destroy_debug_utils_messenger_ext`] **must**  not be called when a callback is active.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`messenger`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`messenger`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-    If [`messenger`] is not [`crate::Handle::null`], [`messenger`] **must**  be a valid [`DebugUtilsMessengerEXT`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`messenger`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`instance`]

## Host Synchronization
- Host access to [`messenger`] **must**  be externally synchronized
The application  **must**  ensure that [`destroy_debug_utils_messenger_ext`] is
not executed in parallel with any Vulkan command that is also called with
[`instance`] or child of [`instance`] as the dispatchable argument.

# Related
- [`ext_debug_utils`]
- [`AllocationCallbacks`]
- [`DebugUtilsMessengerEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        