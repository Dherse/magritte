[VkDescriptorSetAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetAllocateInfo.html) - Structure specifying the allocation parameters for descriptor sets

# C Specifications
The [`DescriptorSetAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorSetAllocateInfo {
    VkStructureType                 sType;
    const void*                     pNext;
    VkDescriptorPool                descriptorPool;
    uint32_t                        descriptorSetCount;
    const VkDescriptorSetLayout*    pSetLayouts;
} VkDescriptorSetAllocateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`descriptor_pool`] is the pool which the sets will be allocated from.
- [`descriptor_set_count`] determines the number of descriptor sets to be allocated from the pool.
- [`set_layouts`] is a pointer to an array of descriptor set layouts, with each member specifying how the corresponding descriptor set is allocated.

# Description
## Valid Usage
-    Each element of [`set_layouts`] **must**  not have been created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set
-    If any element of [`set_layouts`] was created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set, [`descriptor_pool`] **must**  have been created with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` flag set
-    If any element of [`set_layouts`] was created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE` bit set, [`descriptor_pool`] **must**  have been created with the `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` flag set

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`DescriptorSetVariableDescriptorCountAllocateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`descriptor_pool`] **must**  be a valid [`DescriptorPool`] handle
-  [`set_layouts`] **must**  be a valid pointer to an array of [`descriptor_set_count`] valid [`DescriptorSetLayout`] handles
-  [`descriptor_set_count`] **must**  be greater than `0`
-    Both of [`descriptor_pool`], and the elements of [`set_layouts`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPool`]
- [`DescriptorSetLayout`]
- [`StructureType`]
- [`allocate_descriptor_sets`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        