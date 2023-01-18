[vkCmdPushDescriptorSetKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html) - Pushes descriptor updates into a command buffer

# C Specifications
In addition to allocating descriptor sets and binding them to a command
buffer, an application  **can**  record descriptor updates into the command
buffer.To push descriptor updates into a command buffer, call:
```c
// Provided by VK_KHR_push_descriptor
void vkCmdPushDescriptorSetKHR(
    VkCommandBuffer                             commandBuffer,
    VkPipelineBindPoint                         pipelineBindPoint,
    VkPipelineLayout                            layout,
    uint32_t                                    set,
    uint32_t                                    descriptorWriteCount,
    const VkWriteDescriptorSet*                 pDescriptorWrites);
```

# Parameters
- [`command_buffer`] is the command buffer that the descriptors will be recorded in.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline that will use the descriptors. There is a separate set of push descriptor bindings for each pipeline type, so binding one does not disturb the others.
- [`layout`] is a [`PipelineLayout`] object used to program the bindings.
- [`set`] is the set number of the descriptor set in the pipeline layout that will be updated.
- [`descriptor_write_count`] is the number of elements in the [`p_descriptor_writes`] array.
- [`p_descriptor_writes`] is a pointer to an array of [`WriteDescriptorSet`] structures describing the descriptors to be updated.

# Description
*Push descriptors* are a small bank of descriptors whose storage is
internally managed by the command buffer rather than being written into a
descriptor set and later bound to a command buffer.
Push descriptors allow for incremental updates of descriptors without
managing the lifetime of descriptor sets.When a command buffer begins recording, all push descriptors are undefined.
Push descriptors  **can**  be updated incrementally and cause shaders to use the
updated descriptors for subsequent [bound
pipeline commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) with the pipeline type set by [`pipeline_bind_point`]
until the descriptor is overwritten, or else until the set is disturbed as
described in [Pipeline Layout
Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility).
When the set is disturbed or push descriptors with a different descriptor
set layout are set, all push descriptors are undefined.Push descriptors that are [statically used](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-staticuse) by a
pipeline  **must**  not be undefined at the time that a drawing or dispatching
command is recorded to execute using that pipeline.
This includes immutable sampler descriptors, which  **must**  be pushed before
they are accessed by a pipeline (the immutable samplers are pushed, rather
than the samplers in [`p_descriptor_writes`]).
Push descriptors that are not statically used  **can**  remain undefined.Push descriptors do not use dynamic offsets.
Instead, the corresponding non-dynamic descriptor types  **can**  be used and the
`offset` member of [`DescriptorBufferInfo`] **can**  be changed each
time the descriptor is written.Each element of [`p_descriptor_writes`] is interpreted as in
[`WriteDescriptorSet`], except the `dstSet` member is ignored.To push an immutable sampler, use a [`WriteDescriptorSet`] with
`dstBinding` and `dstArrayElement` selecting the immutable sampler’s
binding.
If the descriptor type is `VK_DESCRIPTOR_TYPE_SAMPLER`, the
`pImageInfo` parameter is ignored and the immutable sampler is taken
from the push descriptor set layout in the pipeline layout.
If the descriptor type is `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
the `sampler` member of the `pImageInfo` parameter is ignored and
the immutable sampler is taken from the push descriptor set layout in the
pipeline layout.
## Valid Usage
-  [`pipeline_bind_point`] **must**  be supported by the [`command_buffer`]’s parent [`CommandPool`]’s queue family
-  [`set`] **must**  be less than [`PipelineLayoutCreateInfo::set_layout_count`] provided when [`layout`] was created
-  [`set`] **must**  be the unique set number in the pipeline layout that uses a descriptor set layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
-    For each element i where [`p_descriptor_writes`][i].`descriptorType` is `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, or `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`, [`p_descriptor_writes`][i].`pImageInfo` **must**  be a valid pointer to an array of [`p_descriptor_writes`][i].`descriptorCount` valid [`DescriptorImageInfo`] structures

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`p_descriptor_writes`] **must**  be a valid pointer to an array of [`descriptor_write_count`] valid [`WriteDescriptorSet`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-  [`descriptor_write_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and [`layout`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_push_descriptor`]
- [`CommandBuffer`]
- [`PipelineBindPoint`]
- [`PipelineLayout`]
- [`WriteDescriptorSet`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        