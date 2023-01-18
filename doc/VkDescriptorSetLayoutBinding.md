[VkDescriptorSetLayoutBinding](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBinding.html) - Structure specifying a descriptor set layout binding

# C Specifications
The [`DescriptorSetLayoutBinding`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorSetLayoutBinding {
    uint32_t              binding;
    VkDescriptorType      descriptorType;
    uint32_t              descriptorCount;
    VkShaderStageFlags    stageFlags;
    const VkSampler*      pImmutableSamplers;
} VkDescriptorSetLayoutBinding;
```

# Members
- [`binding`] is the binding number of this entry and corresponds to a resource of the same binding number in the shader stages.
- [`descriptor_type`] is a [`DescriptorType`] specifying which type of resource descriptors are used for this binding.
- [`descriptor_count`] is the number of descriptors contained in the binding, accessed in a shader as an array, except if [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` in which case [`descriptor_count`] is the size in bytes of the inline uniform block. If [`descriptor_count`] is zero this binding entry is reserved and the resource  **must**  not be accessed from any stage via this binding within any pipeline using the set layout.
- [`stage_flags`] member is a bitmask of [`ShaderStageFlagBits`] specifying which pipeline shader stages  **can**  access a resource for this binding. `VK_SHADER_STAGE_ALL` is a shorthand specifying that all defined shader stages, including any additional stages defined by extensions,  **can**  access the resource.If a shader stage is not included in [`stage_flags`], then a resource  **must**  not be accessed from that stage via this binding within any pipeline using the set layout. Other than input attachments which are limited to the fragment shader, there are no limitations on what combinations of stages  **can**  use a descriptor binding, and in particular a binding  **can**  be used by both graphics stages and the compute stage.

# Description
- [`immutable_samplers`] affects initialization of samplers. If [`descriptor_type`] specifies a `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` type descriptor, then [`immutable_samplers`] **can**  be used to initialize a set of *immutable samplers*. Immutable samplers are permanently bound into the set layout and  **must**  not be changed; updating a `VK_DESCRIPTOR_TYPE_SAMPLER` descriptor with immutable samplers is not allowed and updates to a `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` descriptor with immutable samplers does not modify the samplers (the image views are updated, but the sampler updates are ignored). If [`immutable_samplers`] is not `NULL`, then it is a pointer to an array of sampler handles that will be copied into the set layout and used for the corresponding binding. Only the sampler handles are copied; the sampler objects  **must**  not be destroyed before the final use of the set layout and any descriptor pools and sets created using it. If [`immutable_samplers`] is `NULL`, then the sampler slots are dynamic and sampler handles  **must**  be bound into descriptor sets using this layout. If [`descriptor_type`] is not one of these descriptor types, then [`immutable_samplers`] is ignored.
The above layout definition allows the descriptor bindings to be specified
sparsely such that not all binding numbers between 0 and the maximum binding
number need to be specified in the `pBindings` array.
Bindings that are not specified have a [`descriptor_count`] and
[`stage_flags`] of zero, and the value of [`descriptor_type`] is
undefined.
However, all binding numbers between 0 and the maximum binding number in the
[`DescriptorSetLayoutCreateInfo::bindings`] array  **may**  consume
memory in the descriptor set layout even if not all descriptor bindings are
used, though it  **should**  not consume additional memory from the descriptor
pool.
## Valid Usage
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, and [`descriptor_count`] is not `0` and [`immutable_samplers`] is not `NULL`, [`immutable_samplers`] **must**  be a valid pointer to an array of [`descriptor_count`] valid [`Sampler`] handles
-    If the [inlineUniformBlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inlineUniformBlock) feature is not enabled, [`descriptor_type`] **must**  not be `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] **must**  be a multiple of `4`
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] **must**  be less than or equal to [`PhysicalDeviceInlineUniformBlockProperties::max_inline_uniform_block_size`]
-    If [`descriptor_count`] is not `0`, [`stage_flags`] **must**  be a valid combination of [`ShaderStageFlagBits`] values
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and [`descriptor_count`] is not `0`, then [`stage_flags`] **must**  be `0` or `VK_SHADER_STAGE_FRAGMENT_BIT`
-    The sampler objects indicated by [`immutable_samplers`] **must**  not have a `borderColor` with one of the values `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT` or `VK_BORDER_COLOR_INT_CUSTOM_EXT`
-    If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, then [`immutable_samplers`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`descriptor_type`] **must**  be a valid [`DescriptorType`] value

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSetLayoutCreateInfo`]
- [`DescriptorType`]
- [`Sampler`]
- [`ShaderStageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        