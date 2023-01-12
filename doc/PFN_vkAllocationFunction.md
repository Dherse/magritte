[PFN_vkAllocationFunction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkAllocationFunction.html) - Application-defined memory allocation function

# C Specifications
The type of `pfnAllocation` is:
```c
// Provided by VK_VERSION_1_0
typedef void* (VKAPI_PTR *PFN_vkAllocationFunction)(
    void*                                       pUserData,
    size_t                                      size,
    size_t                                      alignment,
    VkSystemAllocationScope                     allocationScope);
```

# Parameters
- [`p_user_data`] is the value specified for [`AllocationCallbacks`]::[`p_user_data`] in the allocator specified by the application.
- [`size`] is the size in bytes of the requested allocation.
- [`alignment`] is the requested alignment of the allocation in bytes and  **must**  be a power of two.
- [`allocation_scope`] is a [`SystemAllocationScope`] value specifying the allocation scope of the lifetime of the allocation, as described [here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-host-allocation-scope).

# Description
If `pfnAllocation` is unable to allocate the requested memory, it  **must** 
return `NULL`.
If the allocation was successful, it  **must**  return a valid pointer to memory
allocation containing at least [`size`] bytes, and with the pointer value
being a multiple of [`alignment`].If `pfnAllocation` returns `NULL`, and if the implementation is unable
to continue correct processing of the current command without the requested
allocation, it  **must**  treat this as a runtime error, and generate
`VK_ERROR_OUT_OF_HOST_MEMORY` at the appropriate time for the command in
which the condition was detected, as described in [Return Codes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-errorcodes).If the implementation is able to continue correct processing of the current
command without the requested allocation, then it  **may**  do so, and  **must**  not
generate `VK_ERROR_OUT_OF_HOST_MEMORY` as a result of this failed
allocation.

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        