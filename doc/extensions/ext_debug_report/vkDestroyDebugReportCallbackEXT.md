[vkDestroyDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDebugReportCallbackEXT.html) - Destroy a debug report callback object

# C Specifications
To destroy a [`DebugReportCallbackEXT`] object, call:
```c
// Provided by VK_EXT_debug_report
void vkDestroyDebugReportCallbackEXT(
    VkInstance                                  instance,
    VkDebugReportCallbackEXT                    callback,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`instance`] is the instance where the callback was created.
- [`callback`] is the [`DebugReportCallbackEXT`] object to destroy. [`callback`] is an externally synchronized object and  **must**  not be used on more than one thread at a time. This means that [`destroy_debug_report_callback_ext`] **must**  not be called when a callback is active.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`callback`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`callback`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-    If [`callback`] is not [`crate::Handle::null`], [`callback`] **must**  be a valid [`DebugReportCallbackEXT`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`callback`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`instance`]

## Host Synchronization
- Host access to [`callback`] **must**  be externally synchronized

# Related
- [`ext_debug_report`]
- [`AllocationCallbacks`]
- [`DebugReportCallbackEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        