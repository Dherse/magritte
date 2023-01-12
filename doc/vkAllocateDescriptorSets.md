[vkAllocateDescriptorSets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateDescriptorSets.html) - Allocate one or more descriptor sets

# C Specifications
To allocate descriptor sets from a descriptor pool, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkAllocateDescriptorSets(
    VkDevice                                    device,
    const VkDescriptorSetAllocateInfo*          pAllocateInfo,
    VkDescriptorSet*                            pDescriptorSets);
```

# Parameters
- [`device`] is the logical device that owns the descriptor pool.
- [`p_allocate_info`] is a pointer to a [`DescriptorSetAllocateInfo`] structure describing parameters of the allocation.
- [`p_descriptor_sets`] is a pointer to an array of [`DescriptorSet`] handles in which the resulting descriptor set objects are returned.

# Description
The allocated descriptor sets are returned in [`p_descriptor_sets`].When a descriptor set is allocated, the initial state is largely
uninitialized and all descriptors are undefined.
Descriptors also become undefined if the underlying resource is destroyed.
Descriptor sets containing undefined descriptors  **can**  still be bound and
used, subject to the following conditions:
- For descriptor set bindings created with the `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` bit set, all descriptors in that binding that are dynamically used  **must**  have been populated before the descriptor set is [consumed](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-binding).
- For descriptor set bindings created without the `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` bit set, all descriptors in that binding that are statically used  **must**  have been populated before the descriptor set is [consumed](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-binding).
- Descriptor bindings with descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` **can**  be undefined when the descriptor set is [consumed](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-binding); though values in that block will be undefined.
- Entries that are not used by a pipeline  **can**  have undefined descriptors.
If a call to [`allocate_descriptor_sets`] would cause the total number of
descriptor sets allocated from the pool to exceed the value of
[`DescriptorPoolCreateInfo::max_sets`] used to create
`pAllocateInfo->descriptorPool`, then the allocation  **may**  fail due to
lack of space in the descriptor pool.
Similarly, the allocation  **may**  fail due to lack of space if the call to
[`allocate_descriptor_sets`] would cause the number of any given
descriptor type to exceed the sum of all the `descriptorCount` members
of each element of [`DescriptorPoolCreateInfo::pool_sizes`] with a
`type` equal to that type.Additionally, the allocation  **may**  also fail if a call to
[`allocate_descriptor_sets`] would cause the total number of inline
uniform block bindings allocated from the pool to exceed the value of
[`DescriptorPoolInlineUniformBlockCreateInfo::max_inline_uniform_block_bindings`]
used to create the descriptor pool.If the allocation fails due to no more space in the descriptor pool, and not
because of system or device memory exhaustion, then
`VK_ERROR_OUT_OF_POOL_MEMORY` **must**  be returned.[`allocate_descriptor_sets`] **can**  be used to create multiple descriptor
sets.
If the creation of any of those descriptor sets fails, then the
implementation  **must**  destroy all successfully created descriptor set objects
from this command, set all entries of the [`p_descriptor_sets`] array to
[`crate::Handle::null`] and return the error.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_allocate_info`] **must**  be a valid pointer to a valid [`DescriptorSetAllocateInfo`] structure
-  [`p_descriptor_sets`] **must**  be a valid pointer to an array of `pAllocateInfo->descriptorSetCount`[`DescriptorSet`] handles
-  `pAllocateInfo->descriptorSetCount` **must**  be greater than `0`

## Host Synchronization
- Host access to `pAllocateInfo->descriptorPool` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_FRAGMENTED_POOL`  - `VK_ERROR_OUT_OF_POOL_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSet`]
- [`DescriptorSetAllocateInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        