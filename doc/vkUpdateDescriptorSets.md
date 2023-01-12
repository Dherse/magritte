[vkUpdateDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateDescriptorSets.html) - Update the contents of a descriptor set object

# C Specifications
Once allocated, descriptor sets  **can**  be updated with a combination of write
and copy operations.
To update descriptor sets, call:
```c
// Provided by VK_VERSION_1_0
void vkUpdateDescriptorSets(
    VkDevice                                    device,
    uint32_t                                    descriptorWriteCount,
    const VkWriteDescriptorSet*                 pDescriptorWrites,
    uint32_t                                    descriptorCopyCount,
    const VkCopyDescriptorSet*                  pDescriptorCopies);
```

# Parameters
- [`device`] is the logical device that updates the descriptor sets.
- [`descriptor_write_count`] is the number of elements in the [`p_descriptor_writes`] array.
- [`p_descriptor_writes`] is a pointer to an array of [`WriteDescriptorSet`] structures describing the descriptor sets to write to.
- [`descriptor_copy_count`] is the number of elements in the [`p_descriptor_copies`] array.
- [`p_descriptor_copies`] is a pointer to an array of [`CopyDescriptorSet`] structures describing the descriptor sets to copy between.

# Description
The operations described by [`p_descriptor_writes`] are performed first,
followed by the operations described by [`p_descriptor_copies`].
Within each array, the operations are performed in the order they appear in
the array.Each element in the [`p_descriptor_writes`] array describes an operation
updating the descriptor set using descriptors for resources specified in the
structure.Each element in the [`p_descriptor_copies`] array is a
[`CopyDescriptorSet`] structure describing an operation copying
descriptors between sets.If the `dstSet` member of any element of [`p_descriptor_writes`] or
[`p_descriptor_copies`] is bound, accessed, or modified by any command that
was recorded to a command buffer which is currently in the
[recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle),
and any of the descriptor bindings that are updated were not created with
the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` or
`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` bits set,
that command buffer becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`, elements of the `pTexelBufferView` member of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`, `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`, `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, the `buffer` member of any element of the `pBufferInfo` member of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and `dstSet` was not allocated with a layout that included immutable samplers for `dstBinding` with `descriptorType`, the `sampler` member of any element of the `pImageInfo` member of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` the `imageView` member of any element of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`, elements of the `pAccelerationStructures` member of a [`WriteDescriptorSetAccelerationStructureKHR`] structure in the `pNext` chain of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`, elements of the `pAccelerationStructures` member of a [`WriteDescriptorSetAccelerationStructureNV`] structure in the `pNext` chain of [`p_descriptor_writes`][i]  **must**  have been created on [`device`]
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, [`p_descriptor_writes`][i].`pImageInfo` **must**  be a valid pointer to an array of [`p_descriptor_writes`][i].`descriptorCount` valid [`DescriptorImageInfo`] structures
-    Descriptor bindings updated by this command which were created without the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` or `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` bits set  **must**  not be used by any command that was recorded to a command buffer which is in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`descriptor_write_count`] is not `0`, [`p_descriptor_writes`] **must**  be a valid pointer to an array of [`descriptor_write_count`] valid [`WriteDescriptorSet`] structures
-    If [`descriptor_copy_count`] is not `0`, [`p_descriptor_copies`] **must**  be a valid pointer to an array of [`descriptor_copy_count`] valid [`CopyDescriptorSet`] structures

## Host Synchronization
- Host access to [`p_descriptor_writes`][].dstSet  **must**  be externally synchronized
- Host access to [`p_descriptor_copies`][].dstSet  **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`CopyDescriptorSet`]
- [`Device`]
- [`WriteDescriptorSet`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        