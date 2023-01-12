[VkDescriptorSetLayoutBindingFlagsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html) - Structure specifying creation flags for descriptor set layout bindings

# C Specifications
If the [`p_next`] chain of a [`DescriptorSetLayoutCreateInfo`]
structure includes a [`DescriptorSetLayoutBindingFlagsCreateInfo`]
structure, then that structure includes an array of flags, one for each
descriptor set layout binding.The [`DescriptorSetLayoutBindingFlagsCreateInfo`] structure is defined
as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
    VkStructureType                    sType;
    const void*                        pNext;
    uint32_t                           bindingCount;
    const VkDescriptorBindingFlags*    pBindingFlags;
} VkDescriptorSetLayoutBindingFlagsCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_descriptor_indexing
typedef VkDescriptorSetLayoutBindingFlagsCreateInfo VkDescriptorSetLayoutBindingFlagsCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`binding_count`] is zero or the number of elements in [`binding_flags`].
- [`binding_flags`] is a pointer to an array of [VkDescriptorBindingFlags]() bitfields, one for each descriptor set layout binding.

# Description
If [`binding_count`] is zero or if this structure is not included in the
[`p_next`] chain, the [VkDescriptorBindingFlags]() for each descriptor
set layout binding is considered to be zero.
Otherwise, the descriptor set layout binding at
[`DescriptorSetLayoutCreateInfo::bindings`][i] uses the flags in
[`binding_flags`][i].
## Valid Usage
-    If [`binding_count`] is not zero, [`binding_count`] **must**  equal [`DescriptorSetLayoutCreateInfo`]::[`binding_count`]
-    If [`DescriptorSetLayoutCreateInfo::flags`] includes `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then all elements of [`binding_flags`] **must**  not include `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`, `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`, or `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
-    If an element of [`binding_flags`] includes `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, then all other elements of [`DescriptorSetLayoutCreateInfo::bindings`] **must**  have a smaller value of `binding`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_uniform_buffer_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_sampled_image_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_storage_image_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_storage_buffer_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_uniform_texel_buffer_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_storage_texel_buffer_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceInlineUniformBlockFeatures::descriptor_binding_inline_uniform_block_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceAccelerationStructureFeaturesKHR::descriptor_binding_acceleration_structure_update_after_bind`] is not enabled, all bindings with descriptor type `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` or `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    All bindings with descriptor type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_update_unused_while_pending`] is not enabled, all elements of [`binding_flags`] **must**  not include `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_partially_bound`] is not enabled, all elements of [`binding_flags`] **must**  not include `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`
-    If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_variable_descriptor_count`] is not enabled, all elements of [`binding_flags`] **must**  not include `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
-    If an element of [`binding_flags`] includes `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, that element’s `descriptorType` **must**  not be `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`
-    If [`binding_count`] is not `0`, [`binding_flags`] **must**  be a valid pointer to an array of [`binding_count`] valid combinations of [`DescriptorBindingFlagBits`] values

# Related
- [`ext_descriptor_indexing`]
- [`crate::vulkan1_2`]
- [VkDescriptorBindingFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        