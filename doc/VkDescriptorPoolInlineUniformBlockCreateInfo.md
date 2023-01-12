[VkDescriptorPoolInlineUniformBlockCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html) - Structure specifying the maximum number of inline uniform block bindings of a newly created descriptor pool

# C Specifications
In order to be able to allocate descriptor sets having
[inline uniform block](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inlineuniformblock) bindings the
descriptor pool  **must**  be created with specifying the inline uniform block
binding capacity of the descriptor pool, in addition to the total inline
uniform data capacity in bytes which is specified through a
[`DescriptorPoolSize`] structure with a `descriptorType` value of
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
This  **can**  be done by adding a
[`DescriptorPoolInlineUniformBlockCreateInfo`] structure to the
[`p_next`] chain of [`DescriptorPoolCreateInfo`].The [`DescriptorPoolInlineUniformBlockCreateInfo`] structure is defined
as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkDescriptorPoolInlineUniformBlockCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           maxInlineUniformBlockBindings;
} VkDescriptorPoolInlineUniformBlockCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_inline_uniform_block
typedef VkDescriptorPoolInlineUniformBlockCreateInfo VkDescriptorPoolInlineUniformBlockCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_inline_uniform_block_bindings`] is the number of inline uniform block bindings to allocate.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`

# Related
- [`ext_inline_uniform_block`]
- [`crate::vulkan1_3`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        