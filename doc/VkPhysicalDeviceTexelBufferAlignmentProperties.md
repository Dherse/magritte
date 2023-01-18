[VkPhysicalDeviceTexelBufferAlignmentProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html) - Structure describing the texel buffer alignment requirements supported by an implementation

# C Specifications
The [`PhysicalDeviceTexelBufferAlignmentProperties`] structure is
defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceTexelBufferAlignmentProperties {
    VkStructureType    sType;
    void*              pNext;
    VkDeviceSize       storageTexelBufferOffsetAlignmentBytes;
    VkBool32           storageTexelBufferOffsetSingleTexelAlignment;
    VkDeviceSize       uniformTexelBufferOffsetAlignmentBytes;
    VkBool32           uniformTexelBufferOffsetSingleTexelAlignment;
} VkPhysicalDeviceTexelBufferAlignmentProperties;
```
or the equivalent
```c
// Provided by VK_EXT_texel_buffer_alignment
typedef VkPhysicalDeviceTexelBufferAlignmentProperties VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`storage_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a storage texel buffer of any format. The value  **must**  be a power of two.
- [`storage_texel_buffer_offset_single_texel_alignment`] indicates whether single texel alignment is sufficient for a storage texel buffer of any format. The value  **must**  be a power of two.
- [`uniform_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a uniform texel buffer of any format. The value  **must**  be a power of two.
- [`uniform_texel_buffer_offset_single_texel_alignment`] indicates whether single texel alignment is sufficient for a uniform texel buffer of any format. The value  **must**  be a power of two.
If the [`PhysicalDeviceTexelBufferAlignmentProperties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.If the single texel alignment property is [`FALSE`], then the buffer
view’s offset  **must**  be aligned to the corresponding byte alignment value.
If the single texel alignment property is [`TRUE`], then the buffer
view’s offset  **must**  be aligned to the lesser of the corresponding byte
alignment value or the size of a single texel, based on
[`BufferViewCreateInfo::format`].
If the size of a single texel is a multiple of three bytes, then the size of
a single component of the format is used instead.These limits  **must**  not advertise a larger alignment than the
[required](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-required) maximum minimum value of
[`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`], for any
format that supports use as a texel buffer.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`

# Related
- [`VK_EXT_texel_buffer_alignment`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        