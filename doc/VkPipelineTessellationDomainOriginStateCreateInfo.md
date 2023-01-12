[VkPipelineTessellationDomainOriginStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) - Structure specifying the orientation of the tessellation domain

# C Specifications
The [`PipelineTessellationDomainOriginStateCreateInfo`] structure is
defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPipelineTessellationDomainOriginStateCreateInfo {
    VkStructureType               sType;
    const void*                   pNext;
    VkTessellationDomainOrigin    domainOrigin;
} VkPipelineTessellationDomainOriginStateCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkPipelineTessellationDomainOriginStateCreateInfo VkPipelineTessellationDomainOriginStateCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`domain_origin`] is a [`TessellationDomainOrigin`] value controlling the origin of the tessellation domain space.

# Description
If the [`PipelineTessellationDomainOriginStateCreateInfo`] structure is
included in the [`p_next`] chain of
[`PipelineTessellationStateCreateInfo`], it controls the origin of the
tessellation domain.
If this structure is not present, it is as if [`domain_origin`] was
`VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`
-  [`domain_origin`] **must**  be a valid [`TessellationDomainOrigin`] value

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]
- [`TessellationDomainOrigin`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        