[VkInternalAllocationType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html) - Allocation type

# C Specifications
The `allocationType` parameter to the `pfnInternalAllocation` and
`pfnInternalFree` functions  **may**  be one of the following values:
```c
// Provided by VK_VERSION_1_0
typedef enum VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
} VkInternalAllocationType;
```

# Description
- [`VK_INTERNAL_ALLOCATION_TYPE`] specifies that the allocation is intended for execution by the host.

# Related
- [`PFNInternalAllocationNotification`]
- [`PFNInternalFreeNotification`]
- [`crate::vulkan1_0`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        