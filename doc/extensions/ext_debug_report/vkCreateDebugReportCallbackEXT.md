[vkCreateDebugReportCallbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugReportCallbackEXT.html) - Create a debug report callback object

# C Specifications
Debug report callbacks give more detailed feedback on the application’s use
of Vulkan when events of interest occur.To register a debug report callback, an application uses
[`create_debug_report_callback_ext`].
```c
// Provided by VK_EXT_debug_report
VkResult vkCreateDebugReportCallbackEXT(
    VkInstance                                  instance,
    const VkDebugReportCallbackCreateInfoEXT*   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDebugReportCallbackEXT*                   pCallback);
```

# Parameters
- [`instance`] is the instance the callback will be logged on.
- [`p_create_info`] is a pointer to a [`DebugReportCallbackCreateInfoEXT`] structure defining the conditions under which this callback will be called.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_callback`] is a pointer to a [`DebugReportCallbackEXT`] handle in which the created object is returned.

# Description
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DebugReportCallbackCreateInfoEXT`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_callback`] **must**  be a valid pointer to a [`DebugReportCallbackEXT`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_EXT_debug_report`]
- [`AllocationCallbacks`]
- [`DebugReportCallbackCreateInfoEXT`]
- [`DebugReportCallbackEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        