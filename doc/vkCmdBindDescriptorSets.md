[vkCmdBindDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindDescriptorSets.html) - Binds descriptor sets to a command buffer

# C Specifications
To bind one or more descriptor sets to a command buffer, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdBindDescriptorSets(
    VkCommandBuffer                             commandBuffer,
    VkPipelineBindPoint                         pipelineBindPoint,
    VkPipelineLayout                            layout,
    uint32_t                                    firstSet,
    uint32_t                                    descriptorSetCount,
    const VkDescriptorSet*                      pDescriptorSets,
    uint32_t                                    dynamicOffsetCount,
    const uint32_t*                             pDynamicOffsets);
```

# Parameters
- [`command_buffer`] is the command buffer that the descriptor sets will be bound to.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline that will use the descriptors. There is a separate set of bind points for each pipeline type, so binding one does not disturb the others.
- [`layout`] is a [`PipelineLayout`] object used to program the bindings.
- [`first_set`] is the set number of the first descriptor set to be bound.
- [`descriptor_set_count`] is the number of elements in the [`p_descriptor_sets`] array.
- [`p_descriptor_sets`] is a pointer to an array of handles to [`DescriptorSet`] objects describing the descriptor sets to bind to.
- [`dynamic_offset_count`] is the number of dynamic offsets in the [`p_dynamic_offsets`] array.
- [`p_dynamic_offsets`] is a pointer to an array of `uint32_t` values specifying dynamic offsets.

# Description
[`cmd_bind_descriptor_sets`] binds descriptor sets
[`p_descriptor_sets`][0..[`descriptor_set_count`]-1] to set numbers
[[`first_set`]..[`first_set`]+[`descriptor_set_count`]-1] for subsequent
[bound pipeline commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) set by
[`pipeline_bind_point`].
Any bindings that were previously applied via these sets are no longer
valid.Once bound, a descriptor set affects rendering of subsequent commands that
interact with the given pipeline type in the command buffer until either a
different set is bound to the same set number, or the set is disturbed as
described in [Pipeline Layout
Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility).A compatible descriptor set  **must**  be bound for all set numbers that any
shaders in a pipeline access, at the time that a drawing or dispatching
command is recorded to execute using that pipeline.
However, if none of the shaders in a pipeline statically use any bindings
with a particular set number, then no descriptor set need be bound for that
set number, even if the pipeline layout includes a non-trivial descriptor
set layout for that set number.If any of the sets being bound include dynamic uniform or storage buffers,
then [`p_dynamic_offsets`] includes one element for each array element in
each dynamic descriptor type binding in each set.
Values are taken from [`p_dynamic_offsets`] in an order such that all
entries for set N come before set N+1; within a set, entries are ordered by
the binding numbers in the descriptor set layouts; and within a binding
array, elements are in order.
[`dynamic_offset_count`] **must**  equal the total number of dynamic descriptors
in the sets being bound.The effective offset used for dynamic uniform and storage buffer bindings is
the sum of the relative offset taken from [`p_dynamic_offsets`], and the
base address of the buffer plus base offset in the descriptor set.
The range of the dynamic uniform and storage buffer bindings is the buffer
range as specified in the descriptor set.Each of the [`p_descriptor_sets`] **must**  be compatible with the pipeline
layout specified by [`layout`].
The layout used to program the bindings  **must**  also be compatible with the
pipeline used in subsequent [bound pipeline
commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) with that pipeline type, as defined in the
[Pipeline Layout Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility) section.The descriptor set contents bound by a call to [`cmd_bind_descriptor_sets`] **may**  be consumed at the following times:
- For descriptor bindings created with the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit set, the contents  **may**  be consumed when the command buffer is submitted to a queue, or during shader execution of the resulting draws and dispatches, or any time in between. Otherwise,
- during host execution of the command, or during shader execution of the resulting draws and dispatches, or any time in between.
Thus, the contents of a descriptor set binding  **must**  not be altered
(overwritten by an update command, or freed) between the first point in time
that it  **may**  be consumed, and when the command completes executing on the
queue.The contents of [`p_dynamic_offsets`] are consumed immediately during
execution of [`cmd_bind_descriptor_sets`].
Once all pending uses have completed, it is legal to update and reuse a
descriptor set.
## Valid Usage
-    Each element of [`p_descriptor_sets`] **must**  have been allocated with a [`DescriptorSetLayout`] that matches (is the same as, or identically defined as) the [`DescriptorSetLayout`] at set *n* in [`layout`], where *n* is the sum of [`first_set`] and the index into [`p_descriptor_sets`]
-  [`dynamic_offset_count`] **must**  be equal to the total number of dynamic descriptors in [`p_descriptor_sets`]
-    The sum of [`first_set`] and [`descriptor_set_count`] **must**  be less than or equal to [`PipelineLayoutCreateInfo::set_layout_count`] provided when [`layout`] was created
-  [`pipeline_bind_point`] **must**  be supported by the [`command_buffer`]’s parent [`CommandPool`]’s queue family
-    Each element of [`p_dynamic_offsets`] which corresponds to a descriptor binding with type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  be a multiple of [`PhysicalDeviceLimits::min_uniform_buffer_offset_alignment`]
-    Each element of [`p_dynamic_offsets`] which corresponds to a descriptor binding with type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  be a multiple of [`PhysicalDeviceLimits::min_storage_buffer_offset_alignment`]
-    For each dynamic uniform or storage buffer binding in [`p_descriptor_sets`], the sum of the effective offset, as defined above, and the range of the binding  **must**  be less than or equal to the size of the buffer
-    Each element of [`p_descriptor_sets`] **must**  not have been allocated from a [`DescriptorPool`] with the `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` flag set

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`p_descriptor_sets`] **must**  be a valid pointer to an array of [`descriptor_set_count`] valid [`DescriptorSet`] handles
-    If [`dynamic_offset_count`] is not `0`, [`p_dynamic_offsets`] **must**  be a valid pointer to an array of [`dynamic_offset_count`]`uint32_t` values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-  [`descriptor_set_count`] **must**  be greater than `0`
-    Each of [`command_buffer`], [`layout`], and the elements of [`p_descriptor_sets`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`DescriptorSet`]
- [`PipelineBindPoint`]
- [`PipelineLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        