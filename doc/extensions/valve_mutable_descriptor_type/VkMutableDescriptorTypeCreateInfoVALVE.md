[VkMutableDescriptorTypeCreateInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html) - Structure describing the list of possible active descriptor types for mutable type descriptors

# C Specifications
Information about the possible descriptor types for mutable descriptor types
is passed in a [`MutableDescriptorTypeCreateInfoVALVE`] structure as a
[`p_next`] to a [`DescriptorSetLayoutCreateInfo`] structure or a
[`DescriptorPoolCreateInfo`] structure.The [`MutableDescriptorTypeCreateInfoVALVE`] structure is defined as:
```c
// Provided by VK_VALVE_mutable_descriptor_type
typedef struct VkMutableDescriptorTypeCreateInfoVALVE {
    VkStructureType                            sType;
    const void*                                pNext;
    uint32_t                                   mutableDescriptorTypeListCount;
    const VkMutableDescriptorTypeListVALVE*    pMutableDescriptorTypeLists;
} VkMutableDescriptorTypeCreateInfoVALVE;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`mutable_descriptor_type_list_count`] is the number of elements in [`mutable_descriptor_type_lists`].
- [`mutable_descriptor_type_lists`] is a pointer to an array of [`MutableDescriptorTypeListVALVE`] structures.

# Description
If [`mutable_descriptor_type_list_count`] is zero or if this structure is not
included in the [`p_next`] chain, the
[`MutableDescriptorTypeListVALVE`] for each element is considered to be
zero or `NULL` for each member.
Otherwise, the descriptor set layout binding at
[`DescriptorSetLayoutCreateInfo::bindings`][i] uses the
descriptor type lists in
[`MutableDescriptorTypeCreateInfoVALVE`]::[`mutable_descriptor_type_lists`][i].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`
-    If [`mutable_descriptor_type_list_count`] is not `0`, [`mutable_descriptor_type_lists`] **must**  be a valid pointer to an array of [`mutable_descriptor_type_list_count`] valid [`MutableDescriptorTypeListVALVE`] structures

# Related
- [`VK_VALVE_mutable_descriptor_type`]
- [`MutableDescriptorTypeListVALVE`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        