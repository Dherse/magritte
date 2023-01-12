[VkDescriptorBindingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) - Bitmask specifying descriptor set layout binding properties

# C Specifications
Bits which  **can**  be set in each element of
[`DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags`],
specifying options for the corresponding descriptor set layout binding, are:
```c
// Provided by VK_VERSION_1_2
typedef enum VkDescriptorBindingFlagBits {
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = 0x00000001,
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = 0x00000002,
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = 0x00000004,
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = 0x00000008,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT = VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT = VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT = VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT,
  // Provided by VK_EXT_descriptor_indexing
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT = VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT,
} VkDescriptorBindingFlagBits;
```
or the equivalent
```c
// Provided by VK_EXT_descriptor_indexing
typedef VkDescriptorBindingFlagBits VkDescriptorBindingFlagBitsEXT;
```

# Description
- [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] indicates that if descriptors in this binding are updated between when the descriptor set is bound in a command buffer and when that command buffer is submitted to a queue, then the submission will use the most recently set descriptors for this binding and the updates do not invalidate the command buffer. Descriptor bindings created with this flag are also partially exempt from the external synchronization requirement in [`update_descriptor_set_with_template_khr`] and [`update_descriptor_sets`]. Multiple descriptors with this flag set  **can**  be updated concurrently in different threads, though the same descriptor  **must**  not be updated concurrently by two threads. Descriptors with this flag set  **can**  be updated concurrently with the set being bound to a command buffer in another thread, but not concurrently with the set being reset or freed.
- [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] indicates that descriptors in this binding that are not *dynamically used* need not contain valid descriptors at the time the descriptors are consumed. A descriptor is dynamically used if any shader invocation executes an instruction that performs any memory access using the descriptor.
- [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] indicates that descriptors in this binding  **can**  be updated after a command buffer has bound this descriptor set, or while a command buffer that uses this descriptor set is pending execution, as long as the descriptors that are updated are not used by those command buffers. If [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] is also set, then descriptors  **can**  be updated as long as they are not dynamically used by any shader invocations. If [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] is not set, then descriptors  **can**  be updated as long as they are not statically used by any shader invocations.
- [`VK_DESCRIPTOR_BINDING_FLAG_BITS`] indicates that     this is a *variable-sized descriptor binding* whose size will be     specified when a descriptor set is allocated using this layout.     The value of `descriptorCount` is treated as an upper bound on the     size of the binding.     This  **must**  only be used for the last binding in the descriptor set     layout (i.e. the binding with the largest value of `binding`).     For the purposes of counting against limits such as     `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value     of `descriptorCount` is     counted, except for descriptor bindings with a descriptor type of     `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.     In this case, `descriptorCount` specifies the upper bound on the     byte size of the binding; thus it counts against the [`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits instead.

# Related
- [`ext_descriptor_indexing`]
- [`crate::vulkan1_2`]
- [VkDescriptorBindingFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        