[VkCopyDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyDescriptorSet.html) - Structure specifying a copy descriptor set operation

# C Specifications
The [`CopyDescriptorSet`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkCopyDescriptorSet {
    VkStructureType    sType;
    const void*        pNext;
    VkDescriptorSet    srcSet;
    uint32_t           srcBinding;
    uint32_t           srcArrayElement;
    VkDescriptorSet    dstSet;
    uint32_t           dstBinding;
    uint32_t           dstArrayElement;
    uint32_t           descriptorCount;
} VkCopyDescriptorSet;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_set`], [`src_binding`], and [`src_array_element`] are the source set, binding, and array element, respectively. If the descriptor binding identified by [`src_set`] and [`src_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`src_array_element`] specifies the starting byte offset within the binding to copy from.
- [`dst_set`], [`dst_binding`], and [`dst_array_element`] are the destination set, binding, and array element, respectively. If the descriptor binding identified by [`dst_set`] and [`dst_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`dst_array_element`] specifies the starting byte offset within the binding to copy to.
- [`descriptor_count`] is the number of descriptors to copy from the source to destination. If [`descriptor_count`] is greater than the number of remaining array elements in the source or destination binding, those affect consecutive bindings in a manner similar to [`WriteDescriptorSet`] above. If the descriptor binding identified by [`src_set`] and [`src_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] specifies the number of bytes to copy and the remaining array elements in the source or destination binding refer to the remaining number of bytes in those.

# Description
If the [`DescriptorSetLayoutBinding`] for [`dst_binding`] is
`VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` and [`src_binding`] is not
`VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the new active descriptor type
becomes the descriptor type of [`src_binding`].
If both [`DescriptorSetLayoutBinding`] for [`src_binding`] and
[`dst_binding`] are `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the active
descriptor type in each source descriptor is copied into the corresponding
destination descriptor.
The active descriptor type  **can**  be different for each source descriptor.
## Valid Usage
-  [`src_binding`] **must**  be a valid binding within [`src_set`]
-    The sum of [`src_array_element`] and [`descriptor_count`] **must**  be less than or equal to the number of array elements in the descriptor set binding specified by [`src_binding`], and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
-  [`dst_binding`] **must**  be a valid binding within [`dst_set`]
-    The sum of [`dst_array_element`] and [`descriptor_count`] **must**  be less than or equal to the number of array elements in the descriptor set binding specified by [`dst_binding`], and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
-    The type of [`dst_binding`] within [`dst_set`] **must**  be equal to the type of [`src_binding`] within [`src_set`]
-    If [`src_set`] is equal to [`dst_set`], then the source and destination ranges of descriptors  **must**  not overlap, where the ranges  **may**  include array elements from consecutive bindings as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
-    If the descriptor type of the descriptor set binding specified by [`src_binding`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`src_array_element`] **must**  be an integer multiple of `4`
-    If the descriptor type of the descriptor set binding specified by [`dst_binding`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`dst_array_element`] **must**  be an integer multiple of `4`
-    If the descriptor type of the descriptor set binding specified by either [`src_binding`] or [`dst_binding`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`descriptor_count`] **must**  be an integer multiple of `4`
-    If [`src_set`]’s layout was created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` flag set, then [`dst_set`]’s layout  **must**  also have been created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` flag set
-    If [`src_set`]’s layout was created with neither `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` nor `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE` flags set, then [`dst_set`]’s layout  **must**  have been created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` flag set
-    If the descriptor pool from which [`src_set`] was allocated was created with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` flag set, then the descriptor pool from which [`dst_set`] was allocated  **must**  also have been created with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` flag set
-    If the descriptor pool from which [`src_set`] was allocated was created with neither `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` nor `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` flags set, then the descriptor pool from which [`dst_set`] was allocated  **must**  have been created without the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` flag set
-    If the descriptor type of the descriptor set binding specified by [`dst_binding`] is `VK_DESCRIPTOR_TYPE_SAMPLER`, then [`dst_set`] **must**  not have been allocated with a layout that included immutable samplers for [`dst_binding`]
-    If [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`] is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the new active descriptor type  **must**  exist in the corresponding `pMutableDescriptorTypeLists` list for [`dst_binding`] if the new active descriptor type is not `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-    If [`DescriptorSetLayoutBinding`] for [`src_set`] at [`src_binding`] is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` and the [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`] is not `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the active descriptor type for the source descriptor  **must**  match the descriptor type of [`dst_binding`]
-    If [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`] is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, and the new active descriptor type is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the `pMutableDescriptorTypeLists` for [`src_binding`] and [`dst_binding`] **must**  match exactly

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET`
-  [`p_next`] **must**  be `NULL`
-  [`src_set`] **must**  be a valid [`DescriptorSet`] handle
-  [`dst_set`] **must**  be a valid [`DescriptorSet`] handle
-    Both of [`dst_set`], and [`src_set`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSet`]
- [`StructureType`]
- [`update_descriptor_sets`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        