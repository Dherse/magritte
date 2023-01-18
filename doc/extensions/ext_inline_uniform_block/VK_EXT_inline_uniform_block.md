[VK_EXT_inline_uniform_block](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_inline_uniform_block.html) - device extension

# Description
This extension introduces the ability to back uniform blocks directly with
descriptor sets by storing inline uniform data within descriptor pool
storage.
Compared to push constants this new construct allows uniform data to be
reused across multiple disjoint sets of drawing or dispatching commands and
 **may**  enable uniform data to be accessed with fewer indirections compared to
uniforms backed by buffer memory.

# Registered extension number
139

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`
- Requires `[`VK_KHR_maintenance1`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Daniel Rakos [aqnuep](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_inline_uniform_block] @aqnuep%0A<<Here describe the issue or question you have about the VK_EXT_inline_uniform_block extension>>)

# New structures
- Extending [`DescriptorPoolCreateInfo`]:  - [`DescriptorPoolInlineUniformBlockCreateInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceInlineUniformBlockFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceInlineUniformBlockPropertiesEXT`] 
- Extending [`WriteDescriptorSet`]:  - [`WriteDescriptorSetInlineUniformBlockEXT`]

# New constants
- [`EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME`]
- [`EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION`]
- Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT`

# Known issues & F.A.Q.
1) Do we need a new storage class for inline uniform blocks vs uniform
blocks? **RESOLVED** : No.
The `Uniform` storage class is used to allow the same syntax used for
both uniform buffers and inline uniform blocks.2) Is the descriptor array index and array size expressed in terms of bytes
or dwords for inline uniform block descriptors? **RESOLVED** : In bytes, but both  **must**  be a multiple of 4, similar to how push
constant ranges are specified.
The `descriptorCount` of [`DescriptorSetLayoutBinding`] thus
provides the total number of bytes a particular binding with an inline
uniform block descriptor type can hold, while the `srcArrayElement`,
`dstArrayElement`, and `descriptorCount` members of
[`WriteDescriptorSet`], [`CopyDescriptorSet`], and
[`DescriptorUpdateTemplateEntry`] (where applicable) specify the byte
offset and number of bytes to write/copy to the binding’s backing store.
Additionally, the `stride` member of
[`DescriptorUpdateTemplateEntry`] is ignored for inline uniform blocks
and a default value of one is used, meaning that the data to update inline
uniform block bindings with must be contiguous in memory.3) What layout rules apply for uniform blocks corresponding to inline
constants? **RESOLVED** : They use the same layout rules as uniform buffers.4) Do we need to add non-uniform indexing features/properties as introduced
by [`VK_EXT_descriptor_indexing`] for inline uniform blocks? **RESOLVED** : No, because inline uniform blocks are not allowed to be
“arrayed”.
A single binding with an inline uniform block descriptor type corresponds to
a single uniform block instance and the array indices inside that binding
refer to individual offsets within the uniform block (see issue #2).
However, this extension does introduce new features/properties about the
level of support for update-after-bind inline uniform blocks.5) Is the `descriptorBindingVariableDescriptorCount` feature introduced
by [`VK_EXT_descriptor_indexing`] supported for inline uniform blocks? **RESOLVED** : Yes, as long as other inline uniform block specific limits are
respected.6) Do the robustness guarantees of `robustBufferAccess` apply to inline
uniform block accesses? **RESOLVED** : No, similarly to push constants, as they are not backed by
buffer memory like uniform buffers.

# Version history
- Revision 1, 2018-08-01 (Daniel Rakos)  - Internal revisions

# Other information
* 2018-08-01
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Daniel Rakos, AMD  - Jeff Bolz, NVIDIA  - Slawomir Grajewski, Intel  - Neil Henning, Codeplay

# Related
- [`DescriptorPoolInlineUniformBlockCreateInfoEXT`]
- [`PhysicalDeviceInlineUniformBlockFeaturesEXT`]
- [`PhysicalDeviceInlineUniformBlockPropertiesEXT`]
- [`WriteDescriptorSetInlineUniformBlockEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        