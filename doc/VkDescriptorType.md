[VkDescriptorType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html) - Specifies the type of a descriptor in a descriptor set

# C Specifications
The type of descriptors in a descriptor set is specified by
[`WriteDescriptorSet::descriptor_type`], which  **must**  be one of the
values:
```c
// Provided by VK_VERSION_1_0
typedef enum VkDescriptorType {
    VK_DESCRIPTOR_TYPE_SAMPLER = 0,
    VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
  // Provided by VK_VERSION_1_3
    VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK = 1000138000,
  // Provided by VK_KHR_acceleration_structure
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR = 1000150000,
  // Provided by VK_NV_ray_tracing
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
  // Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_TYPE_MUTABLE_VALVE = 1000351000,
  // Provided by VK_EXT_inline_uniform_block
    VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT = VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK,
} VkDescriptorType;
```

# Description
- [`VK_DESCRIPTOR_TYPE`] specifies a [sampler descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampler).
- [`VK_DESCRIPTOR_TYPE`] specifies a [combined image sampler descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler).
- [`VK_DESCRIPTOR_TYPE`] specifies a [sampled image descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage).
- [`VK_DESCRIPTOR_TYPE`] specifies a [storage image descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage).
- [`VK_DESCRIPTOR_TYPE`] specifies a [uniform texel buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer).
- [`VK_DESCRIPTOR_TYPE`] specifies a [storage texel buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer).
- [`VK_DESCRIPTOR_TYPE`] specifies a [uniform buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbuffer).
- [`VK_DESCRIPTOR_TYPE`] specifies a [storage buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer).
- [`VK_DESCRIPTOR_TYPE`] specifies a [dynamic uniform buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).
- [`VK_DESCRIPTOR_TYPE`] specifies a [dynamic storage buffer descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).
- [`VK_DESCRIPTOR_TYPE`] specifies an [input attachment descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inputattachment).
- [`INLINE_UNIFORM_BLOCK`] specifies an [inline uniform block](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inlineuniformblock).
- [`MUTABLE_VALVE`] specifies a [descriptor of mutable type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-mutable).
When a descriptor set is updated via elements of [`WriteDescriptorSet`],
members of `pImageInfo`, `pBufferInfo` and `pTexelBufferView`
are only accessed by the implementation when they correspond to descriptor
type being defined - otherwise they are ignored.
The members accessed are as follows for each descriptor type:
- For [`VK_DESCRIPTOR_TYPE`], only the `sampler` member of each element of [`WriteDescriptorSet::image_info`] is accessed.
- For [`VK_DESCRIPTOR_TYPE`], [`VK_DESCRIPTOR_TYPE`], or [`VK_DESCRIPTOR_TYPE`], only the `imageView` and `imageLayout` members of each element of [`WriteDescriptorSet::image_info`] are accessed.
- For [`VK_DESCRIPTOR_TYPE`], all members of each element of [`WriteDescriptorSet::image_info`] are accessed.
- For [`VK_DESCRIPTOR_TYPE`], [`VK_DESCRIPTOR_TYPE`], [`VK_DESCRIPTOR_TYPE`], or [`VK_DESCRIPTOR_TYPE`], all members of each element of [`WriteDescriptorSet::buffer_info`] are accessed.
- For [`VK_DESCRIPTOR_TYPE`] or [`VK_DESCRIPTOR_TYPE`], each element of [`WriteDescriptorSet::texel_buffer_view`] is accessed.
When updating descriptors with a `descriptorType` of
[`INLINE_UNIFORM_BLOCK`], none of the `pImageInfo`,
`pBufferInfo`, or `pTexelBufferView` members are accessed, instead
the source data of the descriptor update operation is taken from the
[`WriteDescriptorSetInlineUniformBlock`] structure in the `pNext`
chain of [`WriteDescriptorSet`].
When updating descriptors with a `descriptorType` of
[`ACCELERATION_STRUCTURE_KHR`], none of the
`pImageInfo`, `pBufferInfo`, or `pTexelBufferView` members are
accessed, instead the source data of the descriptor update operation is
taken from the [`WriteDescriptorSetAccelerationStructureKHR`] structure
in the `pNext` chain of [`WriteDescriptorSet`].
When updating descriptors with a `descriptorType` of
[`ACCELERATION_STRUCTURE_NV`], none of the
`pImageInfo`, `pBufferInfo`, or `pTexelBufferView` members are
accessed, instead the source data of the descriptor update operation is
taken from the [`WriteDescriptorSetAccelerationStructureNV`] structure
in the `pNext` chain of [`WriteDescriptorSet`].

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPoolSize`]
- [`DescriptorSetLayoutBinding`]
- [`DescriptorUpdateTemplateEntry`]
- [`ImageViewHandleInfoNVX`]
- [`MutableDescriptorTypeListVALVE`]
- [`WriteDescriptorSet`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        