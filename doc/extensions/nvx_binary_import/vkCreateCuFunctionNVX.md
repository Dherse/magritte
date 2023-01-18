[vkCreateCuFunctionNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateCuFunctionNVX.html) - Stub description of vkCreateCuFunctionNVX

# C Specifications
There is currently no specification language written for this command.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_NVX_binary_import
VkResult vkCreateCuFunctionNVX(
    VkDevice                                    device,
    const VkCuFunctionCreateInfoNVX*            pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkCuFunctionNVX*                            pFunction);
```

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`CuFunctionCreateInfoNVX`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_function`] **must**  be a valid pointer to a [`CuFunctionNVX`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_NVX_binary_import`]
- [`AllocationCallbacks`]
- [`CuFunctionCreateInfoNVX`]
- [`CuFunctionNVX`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        