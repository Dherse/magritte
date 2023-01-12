[VkDescriptorSetVariableDescriptorCountAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html) - Structure specifying additional allocation parameters for descriptor sets

# C Specifications
If the [`p_next`] chain of a [`DescriptorSetAllocateInfo`] structure
includes a [`DescriptorSetVariableDescriptorCountAllocateInfo`]
structure, then that structure includes an array of descriptor counts for
variable-sized descriptor bindings, one for each descriptor set being
allocated.The [`DescriptorSetVariableDescriptorCountAllocateInfo`] structure is
defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           descriptorSetCount;
    const uint32_t*    pDescriptorCounts;
} VkDescriptorSetVariableDescriptorCountAllocateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_descriptor_indexing
typedef VkDescriptorSetVariableDescriptorCountAllocateInfo VkDescriptorSetVariableDescriptorCountAllocateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`descriptor_set_count`] is zero or the number of elements in [`descriptor_counts`].
- [`descriptor_counts`] is a pointer to an array of descriptor counts, with each member specifying the number of descriptors in a variable-sized descriptor binding in the corresponding descriptor set being allocated.

# Description
If [`descriptor_set_count`] is zero or this structure is not included in the
[`p_next`] chain, then the variable lengths are considered to be zero.
Otherwise, [`descriptor_counts`][i] is the number of descriptors in the
variable-sized descriptor binding in the corresponding descriptor set
layout.
If the variable-sized descriptor binding in the corresponding descriptor set
layout has a descriptor type of
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
[`descriptor_counts`][i] specifies the binding’s capacity in bytes.
If [`DescriptorSetAllocateInfo::set_layouts`][i] does not include
a variable-sized descriptor binding, then [`descriptor_counts`][i] is
ignored.
## Valid Usage
-    If [`descriptor_set_count`] is not zero, [`descriptor_set_count`] **must**  equal [`DescriptorSetAllocateInfo`]::[`descriptor_set_count`]
-    If [`DescriptorSetAllocateInfo::set_layouts`][i] has a variable-sized descriptor binding, then [`descriptor_counts`][i]  **must**  be less than or equal to the descriptor count specified for that binding when the descriptor set layout was created

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`
-    If [`descriptor_set_count`] is not `0`, [`descriptor_counts`] **must**  be a valid pointer to an array of [`descriptor_set_count`]`uint32_t` values

# Related
- [`ext_descriptor_indexing`]
- [`crate::vulkan1_2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        