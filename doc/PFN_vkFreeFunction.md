[PFN_vkFreeFunction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkFreeFunction.html) - Application-defined memory free function

# C Specifications
The type of `pfnFree` is:
```c
// Provided by VK_VERSION_1_0
typedef void (VKAPI_PTR *PFN_vkFreeFunction)(
    void*                                       pUserData,
    void*                                       pMemory);
```

# Parameters
- [`p_user_data`] is the value specified for [`AllocationCallbacks`]::[`p_user_data`] in the allocator specified by the application.
- [`p_memory`] is the allocation to be freed.

# Description
[`p_memory`] **may**  be `NULL`, which the callback  **must**  handle safely.
If [`p_memory`] is non-`NULL`, it  **must**  be a pointer previously allocated
by `pfnAllocation` or `pfnReallocation`.
The application  **should**  free this memory.

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        