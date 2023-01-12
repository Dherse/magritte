[VkWriteDescriptorSetInlineUniformBlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetInlineUniformBlock.html) - Structure specifying inline uniform block data

# C Specifications
If the `descriptorType` member of [`WriteDescriptorSet`] is
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then the data to write to the
descriptor set is specified through a
[`WriteDescriptorSetInlineUniformBlock`] structure included in the
[`p_next`] chain of [`WriteDescriptorSet`].The [`WriteDescriptorSetInlineUniformBlock`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkWriteDescriptorSetInlineUniformBlock {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           dataSize;
    const void*        pData;
} VkWriteDescriptorSetInlineUniformBlock;
```
or the equivalent
```c
// Provided by VK_EXT_inline_uniform_block
typedef VkWriteDescriptorSetInlineUniformBlock VkWriteDescriptorSetInlineUniformBlockEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`data_size`] is the number of bytes of inline uniform block data pointed to by [`data`].
- [`data`] is a pointer to [`data_size`] number of bytes of data to write to the inline uniform block.

# Description
## Valid Usage
-  [`data_size`] **must**  be an integer multiple of `4`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`
-  [`data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`data_size`] **must**  be greater than `0`

# Related
- [`ext_inline_uniform_block`]
- [`crate::vulkan1_3`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        