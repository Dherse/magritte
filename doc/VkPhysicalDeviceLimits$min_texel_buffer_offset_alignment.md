[`min_texel_buffer_offset_alignment`] is the minimum  **required**  alignment,
in bytes, for the `offset` member of the
[`BufferViewCreateInfo`] structure for texel buffers.
The value  **must**  be a power of two.
If [texelBufferAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-texelBufferAlignment) is enabled,
this limit is equivalent to the maximum of the
[`uniformTexelBufferOffsetAlignmentBytes`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-uniformTexelBufferOffsetAlignmentBytes) and
[`storageTexelBufferOffsetAlignmentBytes`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-storageTexelBufferOffsetAlignmentBytes) members of
[`PhysicalDeviceTexelBufferAlignmentProperties`], but smaller
alignment is  **optionally**  allowed by
[`storageTexelBufferOffsetSingleTexelAlignment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-storageTexelBufferOffsetSingleTexelAlignment) and
[`uniformTexelBufferOffsetSingleTexelAlignment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-uniformTexelBufferOffsetSingleTexelAlignment).
If [texelBufferAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-texelBufferAlignment) is not
enabled,
[`BufferViewCreateInfo`]::`offset` **must**  be a multiple of this
value.