[PFN_vkReallocationFunction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkReallocationFunction.html) - Application-defined memory reallocation function

# C Specifications
The type of `pfnReallocation` is:
```c
// Provided by VK_VERSION_1_0
typedef void* (VKAPI_PTR *PFN_vkReallocationFunction)(
    void*                                       pUserData,
    void*                                       pOriginal,
    size_t                                      size,
    size_t                                      alignment,
    VkSystemAllocationScope                     allocationScope);
```

# Parameters
- [`p_user_data`] is the value specified for [`AllocationCallbacks`]::[`p_user_data`] in the allocator specified by the application.
- [`p_original`] **must**  be either `NULL` or a pointer previously returned by `pfnReallocation` or `pfnAllocation` of a compatible allocator.
- [`size`] is the size in bytes of the requested allocation.
- [`alignment`] is the requested alignment of the allocation in bytes and  **must**  be a power of two.
- [`allocation_scope`] is a [`SystemAllocationScope`] value specifying the allocation scope of the lifetime of the allocation, as described [here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-host-allocation-scope).

# Description
`pfnReallocation` **must**  return an allocation with enough space for
[`size`] bytes, and the contents of the original allocation from bytes
zero to min(original size, new size) - 1 **must**  be preserved in the
returned allocation.
If [`size`] is larger than the old size, the contents of the additional
space are undefined.
If satisfying these requirements involves creating a new allocation, then
the old allocation  **should**  be freed.If [`p_original`] is `NULL`, then `pfnReallocation` **must**  behave
equivalently to a call to [`PFNAllocationFunction`] with the same
parameter values (without [`p_original`]).If [`size`] is zero, then `pfnReallocation` **must**  behave equivalently
to a call to [`PFNFreeFunction`] with the same [`p_user_data`]
parameter value, and `pMemory` equal to [`p_original`].If [`p_original`] is non-`NULL`, the implementation  **must**  ensure that
[`alignment`] is equal to the [`alignment`] used to originally allocate
[`p_original`].If this function fails and [`p_original`] is non-`NULL` the application
 **must**  not free the old allocation.`pfnReallocation` **must**  follow the same
[rules for return values as
[`PFNAllocationFunction`]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#vkAllocationFunction_return_rules).

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        