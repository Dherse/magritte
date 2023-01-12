[VkPhysicalDeviceVulkan13Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Properties.html) - Structure specifying physical device properties for functionality promoted to Vulkan 1.3

# C Specifications
The [`PhysicalDeviceVulkan13Properties`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceVulkan13Properties {
    VkStructureType       sType;
    void*                 pNext;
    uint32_t              minSubgroupSize;
    uint32_t              maxSubgroupSize;
    uint32_t              maxComputeWorkgroupSubgroups;
    VkShaderStageFlags    requiredSubgroupSizeStages;
    uint32_t              maxInlineUniformBlockSize;
    uint32_t              maxPerStageDescriptorInlineUniformBlocks;
    uint32_t              maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks;
    uint32_t              maxDescriptorSetInlineUniformBlocks;
    uint32_t              maxDescriptorSetUpdateAfterBindInlineUniformBlocks;
    uint32_t              maxInlineUniformTotalSize;
    VkBool32              integerDotProduct8BitUnsignedAccelerated;
    VkBool32              integerDotProduct8BitSignedAccelerated;
    VkBool32              integerDotProduct8BitMixedSignednessAccelerated;
    VkBool32              integerDotProduct4x8BitPackedUnsignedAccelerated;
    VkBool32              integerDotProduct4x8BitPackedSignedAccelerated;
    VkBool32              integerDotProduct4x8BitPackedMixedSignednessAccelerated;
    VkBool32              integerDotProduct16BitUnsignedAccelerated;
    VkBool32              integerDotProduct16BitSignedAccelerated;
    VkBool32              integerDotProduct16BitMixedSignednessAccelerated;
    VkBool32              integerDotProduct32BitUnsignedAccelerated;
    VkBool32              integerDotProduct32BitSignedAccelerated;
    VkBool32              integerDotProduct32BitMixedSignednessAccelerated;
    VkBool32              integerDotProduct64BitUnsignedAccelerated;
    VkBool32              integerDotProduct64BitSignedAccelerated;
    VkBool32              integerDotProduct64BitMixedSignednessAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating8BitUnsignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating8BitSignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating16BitUnsignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating16BitSignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating32BitUnsignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating32BitSignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating64BitUnsignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating64BitSignedAccelerated;
    VkBool32              integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated;
    VkDeviceSize          storageTexelBufferOffsetAlignmentBytes;
    VkBool32              storageTexelBufferOffsetSingleTexelAlignment;
    VkDeviceSize          uniformTexelBufferOffsetAlignmentBytes;
    VkBool32              uniformTexelBufferOffsetSingleTexelAlignment;
    VkDeviceSize          maxBufferSize;
} VkPhysicalDeviceVulkan13Properties;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`min_subgroup_size`] is the minimum subgroup size supported by this device. [`min_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`min_subgroup_size`] is a power-of-two. [`min_subgroup_size`] is less than or equal to [`max_subgroup_size`]. [`min_subgroup_size`] is less than or equal to [subgroupSize]().
- [`max_subgroup_size`] is the maximum subgroup size supported by this device. [`max_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`max_subgroup_size`] is a power-of-two. [`max_subgroup_size`] is greater than or equal to [`min_subgroup_size`]. [`max_subgroup_size`] is greater than or equal to [subgroupSize]().
- [`max_compute_workgroup_subgroups`] is the maximum number of subgroups supported by the implementation within a workgroup.
- [`required_subgroup_size_stages`] is a bitfield of what shader stages support having a required subgroup size specified.
- [`max_inline_uniform_block_size`] is the maximum size in bytes of an [inline uniform block]() binding.
- `maxPerStageDescriptorInlineUniformBlock` is the maximum number of inline uniform block bindings that  **can**  be accessible to a single shader stage in a pipeline layout. Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit. Only descriptor bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
- [`max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`] is similar to [`max_per_stage_descriptor_inline_uniform_blocks`] but counts descriptor bindings from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_inline_uniform_blocks`] is the maximum number of inline uniform block bindings that  **can**  be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit. Only descriptor bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
- [`max_descriptor_set_update_after_bind_inline_uniform_blocks`] is similar to [`max_descriptor_set_inline_uniform_blocks`] but counts descriptor bindings from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_inline_uniform_total_size`] is the maximum total size in bytes of all inline uniform block bindings, across all pipeline shader stages and descriptor set numbers, that  **can**  be included in a pipeline layout. Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit.
- [`integer_dot_product8_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product8_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product8_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product4x8_bit_packed_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit unsigned dot product operations from operands packed into 32-bit integers using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product4x8_bit_packed_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit signed dot product operations from operands packed into 32-bit integers using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product4x8_bit_packed_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit mixed signedness dot product operations from operands packed into 32-bit integers using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product16_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product16_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product16_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product32_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product32_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product32_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product64_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product64_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product64_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating8_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit unsigned accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit signed accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 8-bit mixed signedness accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating16_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 16-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating32_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 32-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating64_bit_signed_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated`] is a boolean that will be `VK_TRUE` if the support for 64-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
- [`storage_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a storage texel buffer of any format. The value  **must**  be a power of two.
- [`storage_texel_buffer_offset_single_texel_alignment`] indicates whether single texel alignment is sufficient for a storage texel buffer of any format. The value  **must**  be a power of two.
- [`uniform_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a uniform texel buffer of any format. The value  **must**  be a power of two.
- [`uniform_texel_buffer_offset_single_texel_alignment`] indicates whether single texel alignment is sufficient for a uniform texel buffer of any format. The value  **must**  be a power of two.
- [`max_buffer_size`] is the maximum size [`Buffer`] that  **can**  be created.
If the [`PhysicalDeviceVulkan13Properties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.These properties correspond to Vulkan 1.3 functionality.The members of [`PhysicalDeviceVulkan13Properties`] **must**  have the same
values as the corresponding members of
[`PhysicalDeviceInlineUniformBlockProperties`] and
[`PhysicalDeviceSubgroupSizeControlProperties`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`

# Related
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`DeviceSize`]
- [VkShaderStageFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        