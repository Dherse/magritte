[VkDescriptorSetVariableDescriptorCountLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html) - Structure returning information about whether a descriptor set layout can be supported

# C Specifications
If the [`p_next`] chain of a [`DescriptorSetLayoutSupport`] structure
includes a [`DescriptorSetVariableDescriptorCountLayoutSupport`]
structure, then that structure returns additional information about whether
the descriptor set layout is supported.
```c
// Provided by VK_VERSION_1_2
typedef struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxVariableDescriptorCount;
} VkDescriptorSetVariableDescriptorCountLayoutSupport;
```
or the equivalent
```c
// Provided by VK_EXT_descriptor_indexing
typedef VkDescriptorSetVariableDescriptorCountLayoutSupport VkDescriptorSetVariableDescriptorCountLayoutSupportEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_variable_descriptor_count`] indicates the maximum number of descriptors supported in the highest numbered binding of the layout, if that binding is variable-sized. If the highest numbered binding of the layout has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`max_variable_descriptor_count`] indicates the maximum byte size supported for the binding, if that binding is variable-sized.

# Description
If the [`DescriptorSetLayoutCreateInfo`] structure specified in
[`get_descriptor_set_layout_support`]`::pCreateInfo` includes a
variable-sized descriptor, then `supported` is determined assuming the
requested size of the variable-sized descriptor, and
[`max_variable_descriptor_count`] is set to the maximum size of that
descriptor that  **can**  be successfully created (which is greater than or equal
to the requested size passed in).
If the [`DescriptorSetLayoutCreateInfo`] structure does not include a
variable-sized descriptor, or if the
[`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_variable_descriptor_count`]
feature is not enabled, then [`max_variable_descriptor_count`] is set to
zero.
For the purposes of this command, a variable-sized descriptor binding with a
`descriptorCount` of zero is treated as if the `descriptorCount` is
one, and thus the binding is not ignored and the maximum descriptor count
will be returned.
If the layout is not supported, then the value written to
[`max_variable_descriptor_count`] is undefined.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`

# Related
- [`VK_EXT_descriptor_indexing`]
- [`crate::vulkan1_2`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        