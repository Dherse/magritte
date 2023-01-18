[VkBufferViewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferViewCreateInfo.html) - Structure specifying parameters of a newly created buffer view

# C Specifications
The [`BufferViewCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBufferViewCreateInfo {
    VkStructureType            sType;
    const void*                pNext;
    VkBufferViewCreateFlags    flags;
    VkBuffer                   buffer;
    VkFormat                   format;
    VkDeviceSize               offset;
    VkDeviceSize               range;
} VkBufferViewCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`buffer`] is a [`Buffer`] on which the view will be created.
- [`format`] is a [`Format`] describing the format of the data elements in the buffer.
- [`offset`] is an offset in bytes from the base address of the buffer. Accesses to the buffer view from shaders use addressing that is relative to this starting offset.
- [`range`] is a size in bytes of the buffer view. If [`range`] is equal to [`WHOLE_SIZE`], the range from [`offset`] to the end of the buffer is used. If [`WHOLE_SIZE`] is used and the remaining size of the buffer is not a multiple of the [texel block size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#texel-block-size) of [`format`], the nearest smaller multiple is used.

# Description
## Valid Usage
-  [`offset`] **must**  be less than the size of [`buffer`]
-    If [`range`] is not equal to [`WHOLE_SIZE`], [`range`] **must**  be greater than `0`
-    If [`range`] is not equal to [`WHOLE_SIZE`], [`range`] **must**  be an integer multiple of the texel block size of [`format`]
-    If [`range`] is not equal to [`WHOLE_SIZE`], the number of texel buffer elements given by (⌊[`range`] / (texel block size)⌋ × (texels per block)) where texel block size and texels per block are as defined in the [Compatible Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility) table for [`format`],  **must**  be less than or equal to [`PhysicalDeviceLimits::max_texel_buffer_elements`]
-    If [`range`] is not equal to [`WHOLE_SIZE`], the sum of [`offset`] and [`range`] **must**  be less than or equal to the size of [`buffer`]
-    If [`range`] is equal to [`WHOLE_SIZE`], the number of texel buffer elements given by (⌊(size - [`offset`]) / (texel block size)⌋ × (texels per block)) where size is the size of [`buffer`], and texel block size and texels per block are as defined in the [Compatible Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility) table for [`format`],  **must**  be less than or equal to [`PhysicalDeviceLimits::max_texel_buffer_elements`]
-  [`buffer`] **must**  have been created with a `usage` value containing at least one of `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT` or `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`
-    If [`buffer`] was created with `usage` containing `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT`, [`format`] **must**  be supported for uniform texel buffers, as specified by the `VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT` flag in [`FormatProperties::buffer_features`] returned by [`get_physical_device_format_properties`]
-    If [`buffer`] was created with `usage` containing `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`, [`format`] **must**  be supported for storage texel buffers, as specified by the `VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT` flag in [`FormatProperties::buffer_features`] returned by [`get_physical_device_format_properties`]
-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If the [texelBufferAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-texelBufferAlignment) feature is not enabled, [`offset`] **must**  be a multiple of [`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`]
-    If the [texelBufferAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-texelBufferAlignment) feature is enabled and if [`buffer`] was created with `usage` containing `VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT`, [`offset`] **must**  be a multiple of the lesser of [`PhysicalDeviceTexelBufferAlignmentProperties::storage_texel_buffer_offset_alignment_bytes`] or, if [`PhysicalDeviceTexelBufferAlignmentProperties::storage_texel_buffer_offset_single_texel_alignment`] is [`TRUE`], the size of a texel of the requested [`format`]. If the size of a texel is a multiple of three bytes, then the size of a single component of [`format`] is used instead
-    If the [texelBufferAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-texelBufferAlignment) feature is enabled and if [`buffer`] was created with `usage` containing `VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT`, [`offset`] **must**  be a multiple of the lesser of [`PhysicalDeviceTexelBufferAlignmentProperties::uniform_texel_buffer_offset_alignment_bytes`] or, if [`PhysicalDeviceTexelBufferAlignmentProperties::uniform_texel_buffer_offset_single_texel_alignment`] is [`TRUE`], the size of a texel of the requested [`format`]. If the size of a texel is a multiple of three bytes, then the size of a single component of [`format`] is used instead

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`format`] **must**  be a valid [`Format`] value

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`BufferViewCreateFlags`]
- [`DeviceSize`]
- [`Format`]
- [`StructureType`]
- [`create_buffer_view`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        