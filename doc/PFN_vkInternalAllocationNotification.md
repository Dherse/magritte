[PFN_vkInternalAllocationNotification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/PFN_vkInternalAllocationNotification.html) - Application-defined memory allocation notification function

# C Specifications
The type of `pfnInternalAllocation` is:
```c
// Provided by VK_VERSION_1_0
typedef void (VKAPI_PTR *PFN_vkInternalAllocationNotification)(
    void*                                       pUserData,
    size_t                                      size,
    VkInternalAllocationType                    allocationType,
    VkSystemAllocationScope                     allocationScope);
```

# Parameters
- [`p_user_data`] is the value specified for [`AllocationCallbacks`]::[`p_user_data`] in the allocator specified by the application.
- [`size`] is the requested size of an allocation.
- [`allocation_type`] is a [`InternalAllocationType`] value specifying the requested type of an allocation.
- [`allocation_scope`] is a [`SystemAllocationScope`] value specifying the allocation scope of the lifetime of the allocation, as described [here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-host-allocation-scope).

# Description
This is a purely informational callback.

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        