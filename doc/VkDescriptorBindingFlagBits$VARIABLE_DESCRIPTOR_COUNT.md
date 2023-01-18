[`VARIABLE_DESCRIPTOR_COUNT`] indicates that
    this is a *variable-sized descriptor binding* whose size will be
    specified when a descriptor set is allocated using this layout.
    The value of `descriptorCount` is treated as an upper bound on the
    size of the binding.
    This  **must**  only be used for the last binding in the descriptor set
    layout (i.e. the binding with the largest value of `binding`).
    For the purposes of counting against limits such as
    `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value
    of `descriptorCount` is
    counted, except for descriptor bindings with a descriptor type of
    `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
    In this case, `descriptorCount` specifies the upper bound on the
    byte size of the binding; thus it counts against the
[`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits
instead.