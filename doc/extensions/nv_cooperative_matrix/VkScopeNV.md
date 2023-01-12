[VkScopeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScopeNV.html) - Specify SPIR-V scope

# C Specifications
Possible values for [`ScopeNV`] include:
```c
// Provided by VK_NV_cooperative_matrix
typedef enum VkScopeNV {
    VK_SCOPE_DEVICE_NV = 1,
    VK_SCOPE_WORKGROUP_NV = 2,
    VK_SCOPE_SUBGROUP_NV = 3,
    VK_SCOPE_QUEUE_FAMILY_NV = 5,
} VkScopeNV;
```

# Description
- [`VK_SCOPE_NV`] corresponds to SPIR-V [`Device`] scope.
- [`VK_SCOPE_NV`] corresponds to SPIR-V `Workgroup` scope.
- [`VK_SCOPE_NV`] corresponds to SPIR-V `Subgroup` scope.
- [`VK_SCOPE_NV`] corresponds to SPIR-V `QueueFamily` scope.
All enum values match the corresponding SPIR-V value.

# Related
- [`nv_cooperative_matrix`]
- [`CooperativeMatrixPropertiesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        