[vkCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) - Create a debug messenger object

# C Specifications
A debug messenger triggers a debug callback with a debug message when an
event of interest occurs.
To create a debug messenger which will trigger a debug callback, call:
```c
// Provided by VK_EXT_debug_utils
VkResult vkCreateDebugUtilsMessengerEXT(
    VkInstance                                  instance,
    const VkDebugUtilsMessengerCreateInfoEXT*   pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDebugUtilsMessengerEXT*                   pMessenger);
```

# Parameters
- [`instance`] is the instance the messenger will be used with.
- [`p_create_info`] is a pointer to a [`DebugUtilsMessengerCreateInfoEXT`] structure containing the callback pointer, as well as defining conditions under which this messenger will trigger the callback.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_messenger`] is a pointer to a [`DebugUtilsMessengerEXT`] handle in which the created object is returned.

# Description
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DebugUtilsMessengerCreateInfoEXT`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_messenger`] **must**  be a valid pointer to a [`DebugUtilsMessengerEXT`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY` 
The application  **must**  ensure that [`create_debug_utils_messenger_ext`] is
not executed in parallel with any Vulkan command that is also called with
[`instance`] or child of [`instance`] as the dispatchable argument.

# Related
- [`VK_EXT_debug_utils`]
- [`AllocationCallbacks`]
- [`DebugUtilsMessengerCreateInfoEXT`]
- [`DebugUtilsMessengerEXT`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        