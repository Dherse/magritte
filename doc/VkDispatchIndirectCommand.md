[VkDispatchIndirectCommand](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDispatchIndirectCommand.html) - Structure specifying a indirect dispatching command

# C Specifications
The [`DispatchIndirectCommand`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDispatchIndirectCommand {
    uint32_t    x;
    uint32_t    y;
    uint32_t    z;
} VkDispatchIndirectCommand;
```

# Members
- [`x`] is the number of local workgroups to dispatch in the X dimension.
- [`y`] is the number of local workgroups to dispatch in the Y dimension.
- [`z`] is the number of local workgroups to dispatch in the Z dimension.

# Description
The members of [`DispatchIndirectCommand`] have the same meaning as the
corresponding parameters of [`cmd_dispatch`].
## Valid Usage
-  [`x`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][0]
-  [`y`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][1]
-  [`z`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][2]

# Related
- [`crate::vulkan1_0`]
- [`cmd_dispatch_indirect`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        