[VkWriteDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSet.html) - Structure specifying the parameters of a descriptor set write operation

# C Specifications
The [`WriteDescriptorSet`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkWriteDescriptorSet {
    VkStructureType                  sType;
    const void*                      pNext;
    VkDescriptorSet                  dstSet;
    uint32_t                         dstBinding;
    uint32_t                         dstArrayElement;
    uint32_t                         descriptorCount;
    VkDescriptorType                 descriptorType;
    const VkDescriptorImageInfo*     pImageInfo;
    const VkDescriptorBufferInfo*    pBufferInfo;
    const VkBufferView*              pTexelBufferView;
} VkWriteDescriptorSet;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`dst_set`] is the destination descriptor set to update.
- [`dst_binding`] is the descriptor binding within that set.
- [`dst_array_element`] is the starting element in that array. If the descriptor binding identified by [`dst_set`] and [`dst_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`dst_array_element`] specifies the starting byte offset within the binding.
- [`descriptor_count`] is the number of descriptors to update. If the descriptor binding identified by [`dst_set`] and [`dst_binding`] has a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, then [`descriptor_count`] specifies the number of bytes to update. Otherwise, [`descriptor_count`] is one of  - the number of elements in [`image_info`]  - the number of elements in [`buffer_info`]  - the number of elements in [`texel_buffer_view`]  - a value matching the `dataSize` member of a [`WriteDescriptorSetInlineUniformBlock`] structure in the [`p_next`] chain  - a value matching the `accelerationStructureCount` of a [`WriteDescriptorSetAccelerationStructureKHR`] structure in the [`p_next`] chain 
- [`descriptor_type`] is a [`DescriptorType`] specifying the type of each descriptor in [`image_info`], [`buffer_info`], or [`texel_buffer_view`], as described below. If [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`] is not equal to `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, [`descriptor_type`] **must**  be the same type as the [`descriptor_type`] specified in [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`]. The type of the descriptor also controls which array the descriptors are taken from.
- [`image_info`] is a pointer to an array of [`DescriptorImageInfo`] structures or is ignored, as described below.
- [`buffer_info`] is a pointer to an array of [`DescriptorBufferInfo`] structures or is ignored, as described below.
- [`texel_buffer_view`] is a pointer to an array of [`BufferView`] handles as described in the [Buffer Views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-buffer-views) section or is ignored, as described below.

# Description
Only one of [`image_info`], [`buffer_info`], or [`texel_buffer_view`]
members is used according to the descriptor type specified in the
[`descriptor_type`] member of the containing [`WriteDescriptorSet`]
structure,
or none of them in case [`descriptor_type`] is
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, in which case the source data
for the descriptor writes is taken from the
[`WriteDescriptorSetInlineUniformBlock`] structure included in the
[`p_next`] chain of [`WriteDescriptorSet`],
or if [`descriptor_type`] is
`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`, in which case the
source data for the descriptor writes is taken from the
[`WriteDescriptorSetAccelerationStructureKHR`] structure in the
[`p_next`] chain of [`WriteDescriptorSet`],
or if [`descriptor_type`] is
`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`, in which case the source
data for the descriptor writes is taken from the
[`WriteDescriptorSetAccelerationStructureNV`] structure in the
[`p_next`] chain of [`WriteDescriptorSet`],
as specified below.If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is enabled, the
buffer,
acceleration structure,
imageView, or bufferView  **can**  be [`crate::Handle::null`].
Loads from a null descriptor return zero values and stores and atomics to a
null descriptor are discarded.
A null acceleration structure descriptor results in the miss shader being
invoked.If the destination descriptor is a mutable descriptor, the active descriptor
type for the destination descriptor becomes [`descriptor_type`].If the [`dst_binding`] has fewer than [`descriptor_count`] array elements
remaining starting from [`dst_array_element`], then the remainder will be
used to update the subsequent binding - [`dst_binding`]+1 starting at
array element zero.
If a binding has a [`descriptor_count`] of zero, it is skipped.
This behavior applies recursively, with the update affecting consecutive
bindings as needed to update all [`descriptor_count`] descriptors.
Consecutive bindings  **must**  have identical [`DescriptorType`],
[VkShaderStageFlags](),
[`DescriptorBindingFlagBits`],
and immutable samplers references.
## Valid Usage
-  [`dst_binding`] **must**  be less than or equal to the maximum value of `binding` of all [`DescriptorSetLayoutBinding`] structures specified when [`dst_set`]’s descriptor set layout was created
-  [`dst_binding`] **must**  be a binding with a non-zero [`descriptor_count`]
-    All consecutive bindings updated via a single [`WriteDescriptorSet`] structure, except those with a [`descriptor_count`] of zero,  **must**  have identical [`descriptor_type`] and `stageFlags`
-    All consecutive bindings updated via a single [`WriteDescriptorSet`] structure, except those with a [`descriptor_count`] of zero,  **must**  all either use immutable samplers or  **must**  all not use immutable samplers
-  [`descriptor_type`] **must**  match the type of [`dst_binding`] within [`dst_set`]
-  [`dst_set`] **must**  be a valid [`DescriptorSet`] handle
-    The sum of [`dst_array_element`] and [`descriptor_count`] **must**  be less than or equal to the number of array elements in the descriptor set binding specified by [`dst_binding`], and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`dst_array_element`] **must**  be an integer multiple of `4`
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`descriptor_count`] **must**  be an integer multiple of `4`
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`, each element of [`texel_buffer_view`] **must**  be either a valid [`BufferView`] handle or [`crate::Handle::null`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` and the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is not enabled, each element of [`texel_buffer_view`] **must**  not be [`crate::Handle::null`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`, `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`, `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, [`buffer_info`] **must**  be a valid pointer to an array of [`descriptor_count`] valid [`DescriptorBufferInfo`] structures
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and [`dst_set`] was not allocated with a layout that included immutable samplers for [`dst_binding`] with [`descriptor_type`], the `sampler` member of each element of [`image_info`] **must**  be a valid [`Sampler`] object
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, the `imageView` member of each element of [`image_info`] **must**  be either a valid [`ImageView`] handle or [`crate::Handle::null`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is not enabled, the `imageView` member of each element of [`image_info`] **must**  not be [`crate::Handle::null`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, the [`p_next`] chain  **must**  include a [`WriteDescriptorSetInlineUniformBlock`] structure whose `dataSize` member equals [`descriptor_count`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`, the [`p_next`] chain  **must**  include a [`WriteDescriptorSetAccelerationStructureKHR`] structure whose `accelerationStructureCount` member equals [`descriptor_count`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`, the [`p_next`] chain  **must**  include a [`WriteDescriptorSetAccelerationStructureNV`] structure whose `accelerationStructureCount` member equals [`descriptor_count`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, then the `imageView` member of each [`image_info`] element  **must**  have been created without a [`SamplerYcbcrConversionInfo`] structure in its [`p_next`] chain
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and if any element of [`image_info`] has a `imageView` member that was created with a [`SamplerYcbcrConversionInfo`] structure in its [`p_next`] chain, then [`dst_set`] **must**  have been allocated with a layout that included immutable samplers for [`dst_binding`], and the corresponding immutable sampler  **must**  have been created with an *identically defined*[`SamplerYcbcrConversionInfo`] object
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and [`dst_set`] was allocated with a layout that included immutable samplers for [`dst_binding`], then the `imageView` member of each element of [`image_info`] which corresponds to an immutable sampler that enables [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) **must**  have been created with a [`SamplerYcbcrConversionInfo`] structure in its [`p_next`] chain with an *identically defined*[`SamplerYcbcrConversionInfo`] to the corresponding immutable sampler
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, the `offset` member of each element of [`buffer_info`] **must**  be a multiple of [`PhysicalDeviceLimits::min_uniform_buffer_offset_alignment`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, the `offset` member of each element of [`buffer_info`] **must**  be a multiple of [`PhysicalDeviceLimits::min_storage_buffer_offset_alignment`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`, `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`, or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, and the `buffer` member of any element of [`buffer_info`] is the handle of a non-sparse buffer, then that buffer  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, the `buffer` member of each element of [`buffer_info`] **must**  have been created with `VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, the `buffer` member of each element of [`buffer_info`] **must**  have been created with `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, the `range` member of each element of [`buffer_info`], or the effective range if `range` is `VK_WHOLE_SIZE`,  **must**  be less than or equal to [`PhysicalDeviceLimits::max_uniform_buffer_range`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, the `range` member of each element of [`buffer_info`], or the effective range if `range` is `VK_WHOLE_SIZE`,  **must**  be less than or equal to [`PhysicalDeviceLimits::max_storage_buffer_range`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`, the [`Buffer`] that each element of [`texel_buffer_view`] was created from  **must**  have been created with `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`, the [`Buffer`] that each element of [`texel_buffer_view`] was created from  **must**  have been created with `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, the `imageView` member of each element of [`image_info`] **must**  have been created with the identity swizzle
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, the `imageView` member of each element of [`image_info`] **must**  have been created with `VK_IMAGE_USAGE_SAMPLED_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` the `imageLayout` member of each element of [`image_info`] **must**  be a member of the list given in [Sampled Image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage)
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` the `imageLayout` member of each element of [`image_info`] **must**  be a member of the list given in [Combined Image Sampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler)
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` the `imageLayout` member of each element of [`image_info`] **must**  be a member of the list given in [Input Attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inputattachment)
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` the `imageLayout` member of each element of [`image_info`] **must**  be a member of the list given in [Storage Image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage)
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, the `imageView` member of each element of [`image_info`] **must**  have been created with `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, the `imageView` member of each element of [`image_info`] **must**  have been created with `VK_IMAGE_USAGE_STORAGE_BIT` set
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLER`, then [`dst_set`] **must**  not have been allocated with a layout that included immutable samplers for [`dst_binding`]
-    If the [`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`] is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, the new active descriptor type [`descriptor_type`] **must**  exist in the corresponding `pMutableDescriptorTypeLists` list for [`dst_binding`]
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, the `imageView` member of each element of [`image_info`] **must**  have either been created without a [`ImageViewMinLodCreateInfoEXT`] present in the [`p_next`] chain or with a [`ImageViewMinLodCreateInfoEXT::min_lod`] of `0.0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`WriteDescriptorSetAccelerationStructureKHR`], [`WriteDescriptorSetAccelerationStructureNV`], or [`WriteDescriptorSetInlineUniformBlock`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`descriptor_type`] **must**  be a valid [`DescriptorType`] value
-  [`descriptor_count`] **must**  be greater than `0`
-    Both of [`dst_set`], and the elements of [`texel_buffer_view`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`BufferView`]
- [`DescriptorBufferInfo`]
- [`DescriptorImageInfo`]
- [`DescriptorSet`]
- [`DescriptorType`]
- [`StructureType`]
- [`cmd_push_descriptor_set_khr`]
- [`update_descriptor_sets`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        