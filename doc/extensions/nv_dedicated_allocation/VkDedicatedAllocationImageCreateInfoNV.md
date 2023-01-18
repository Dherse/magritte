[VkDedicatedAllocationImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDedicatedAllocationImageCreateInfoNV.html) - Specify that an image is bound to a dedicated memory resource

# C Specifications
If the [`p_next`] chain includes a
[`DedicatedAllocationImageCreateInfoNV`] structure, then that structure
includes an enable controlling whether the image will have a dedicated
memory allocation bound to it.The [`DedicatedAllocationImageCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_dedicated_allocation
typedef struct VkDedicatedAllocationImageCreateInfoNV {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           dedicatedAllocation;
} VkDedicatedAllocationImageCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`dedicated_allocation`] specifies whether the image will have a dedicated allocation bound to it.

# Description
## Valid Usage
-    If [`dedicated_allocation`] is [`TRUE`], [`ImageCreateInfo::flags`] **must**  not include `VK_IMAGE_CREATE_SPARSE_BINDING_BIT`, `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT`, or `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV`

# Related
- [`VK_NV_dedicated_allocation`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        