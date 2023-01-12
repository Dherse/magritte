[VkDescriptorUpdateTemplateEntry](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) - Describes a single descriptor update of the descriptor update template

# C Specifications
The [`DescriptorUpdateTemplateEntry`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkDescriptorUpdateTemplateEntry {
    uint32_t            dstBinding;
    uint32_t            dstArrayElement;
    uint32_t            descriptorCount;
    VkDescriptorType    descriptorType;
    size_t              offset;
    size_t              stride;
} VkDescriptorUpdateTemplateEntry;
```
or the equivalent
```c
// Provided by VK_KHR_descriptor_update_template
typedef VkDescriptorUpdateTemplateEntry VkDescriptorUpdateTemplateEntryKHR;
```

# Members
- [`dst_binding`] is the descriptor binding to update when using this descriptor update template.
- [`dst_array_element`] is the starting element in the array belonging to [`dst_binding`]. If the descriptor binding identified by [`dst_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`dst_array_element`] specifies the starting byte offset to update.
- [`descriptor_count`] is the number of descriptors to update. If [`descriptor_count`] is greater than the number of remaining array elements in the destination binding, those affect consecutive bindings in a manner similar to [`WriteDescriptorSet`] above. If the descriptor binding identified by [`dst_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] specifies the number of bytes to update and the remaining array elements in the destination binding refer to the remaining number of bytes in it.
- [`descriptor_type`] is a [`DescriptorType`] specifying the type of the descriptor.
- [`offset`] is the offset in bytes of the first binding in the raw data structure.
- [`stride`] is the stride in bytes between two consecutive array elements of the descriptor update informations in the raw data structure. The actual pointer ptr for each array element j of update entry i is computed using the following formula: ```c     const char *ptr = (const char *)pData + pDescriptorUpdateEntries[i].offset + j * pDescriptorUpdateEntries[i].stride ``` The stride is useful in case the bindings are stored in structs along with other data. If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then the value of [`stride`] is ignored and the stride is assumed to be `1`, i.e. the descriptor update information for them is always specified as a contiguous range.

# Description
## Valid Usage
-  [`dst_binding`] **must**  be a valid binding in the descriptor set layout implicitly specified when using a descriptor update template to update descriptors
-  [`dst_array_element`] and [`descriptor_count`] **must**  be less than or equal to the number of array elements in the descriptor set binding implicitly specified when using a descriptor update template to update descriptors, and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
-    If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`dst_array_element`] **must**  be an integer multiple of `4`
-    If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`descriptor_count`] **must**  be an integer multiple of `4`

## Valid Usage (Implicit)
-  [`descriptor_type`] **must**  be a valid [`DescriptorType`] value

# Related
- [`crate::vulkan1_1`]
- [`DescriptorType`]
- [`DescriptorUpdateTemplateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        