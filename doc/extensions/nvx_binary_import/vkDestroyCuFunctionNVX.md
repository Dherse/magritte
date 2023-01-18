[vkDestroyCuFunctionNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyCuFunctionNVX.html) - Stub description of vkDestroyCuFunctionNVX

# C Specifications
There is currently no specification language written for this command.
This section acts only as placeholder and to avoid dead links in the
specification and reference pages.
```c
// Provided by VK_NVX_binary_import
void vkDestroyCuFunctionNVX(
    VkDevice                                    device,
    VkCuFunctionNVX                             function,
    const VkAllocationCallbacks*                pAllocator);
```

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`function`] **must**  be a valid [`CuFunctionNVX`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`function`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_NVX_binary_import`]
- [`AllocationCallbacks`]
- [`CuFunctionNVX`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        