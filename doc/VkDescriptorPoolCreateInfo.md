[VkDescriptorPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateInfo.html) - Structure specifying parameters of a newly created descriptor pool

# C Specifications
Additional information about the pool is passed in a
[`DescriptorPoolCreateInfo`] structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDescriptorPoolCreateInfo {
    VkStructureType                sType;
    const void*                    pNext;
    VkDescriptorPoolCreateFlags    flags;
    uint32_t                       maxSets;
    uint32_t                       poolSizeCount;
    const VkDescriptorPoolSize*    pPoolSizes;
} VkDescriptorPoolCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`DescriptorPoolCreateFlagBits`] specifying certain supported operations on the pool.
- [`max_sets`] is the maximum number of descriptor sets that  **can**  be allocated from the pool.
- [`pool_size_count`] is the number of elements in [`pool_sizes`].
- [`pool_sizes`] is a pointer to an array of [`DescriptorPoolSize`] structures, each containing a descriptor type and number of descriptors of that type to be allocated in the pool.

# Description
If multiple [`DescriptorPoolSize`] structures containing the same
descriptor type appear in the [`pool_sizes`] array then the pool will be
created with enough storage for the total number of descriptors of each
type.Fragmentation of a descriptor pool is possible and  **may**  lead to descriptor
set allocation failures.
A failure due to fragmentation is defined as failing a descriptor set
allocation despite the sum of all outstanding descriptor set allocations
from the pool plus the requested allocation requiring no more than the total
number of descriptors requested at pool creation.
Implementations provide certain guarantees of when fragmentation  **must**  not
cause allocation failure, as described below.If a descriptor pool has not had any descriptor sets freed since it was
created or most recently reset then fragmentation  **must**  not cause an
allocation failure (note that this is always the case for a pool created
without the `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT` bit
set).
Additionally, if all sets allocated from the pool since it was created or
most recently reset use the same number of descriptors (of each type) and
the requested allocation also uses that same number of descriptors (of each
type), then fragmentation  **must**  not cause an allocation failure.If an allocation failure occurs due to fragmentation, an application  **can** 
create an additional descriptor pool to perform further descriptor set
allocations.If [`flags`] has the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT`
bit set, descriptor pool creation  **may**  fail with the error
`VK_ERROR_FRAGMENTATION` if the total number of descriptors across all
pools (including this one) created with this bit set exceeds
`maxUpdateAfterBindDescriptorsInAllPools`, or if fragmentation of the
underlying hardware resources occurs.If a [`pool_sizes`][i]::`type` is
`VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, a
[`MutableDescriptorTypeCreateInfoVALVE`] struct in the [`p_next`] chain
 **can**  be used to specify which mutable descriptor types  **can**  be allocated
from the pool.
If present in the [`p_next`] chain,
[`MutableDescriptorTypeCreateInfoVALVE::mutable_descriptor_type_lists`][i]
specifies which kind of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` descriptors
 **can**  be allocated from this pool entry.
If [`MutableDescriptorTypeCreateInfoVALVE`] does not exist in the
[`p_next`] chain, or
[`MutableDescriptorTypeCreateInfoVALVE::mutable_descriptor_type_lists`][i]
is out of range, the descriptor pool allocates enough memory to be able to
allocate a `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` descriptor with any
supported [`DescriptorType`] as a mutable descriptor.
A mutable descriptor  **can**  be allocated from a pool entry if the type list in
[`DescriptorSetLayoutCreateInfo`] is a subset of the type list declared
in the descriptor pool, or if the pool entry is created without a descriptor
type list.
Multiple [`pool_sizes`] entries with
`VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` **can**  be declared.
When multiple such pool entries are present in [`pool_sizes`], they
specify sets of supported descriptor types which either fully overlap,
partially overlap, or are disjoint.
Two sets fully overlap if the sets of supported descriptor types are equal.
If the sets are not disjoint they partially overlap.
A pool entry without a [`MutableDescriptorTypeListVALVE`] assigned to it
is considered to partially overlap any other pool entry which has a
[`MutableDescriptorTypeListVALVE`] assigned to it.
The application  **must**  ensure that partial overlap does not exist in
[`pool_sizes`].
## Valid Usage
-  [`max_sets`] **must**  be greater than `0`
-    If [`flags`] has the `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` bit set, then the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit  **must**  not be set
-    If [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::mutable_descriptor_type`] is not enabled, [`pool_sizes`] **must**  not contain a `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
-    If [`flags`] has the `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` bit set, [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE::mutable_descriptor_type`] **must**  be enabled
-    If [`pool_sizes`] contains a `descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`, any other `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` element in [`pool_sizes`] **must**  not have sets of supported descriptor types which partially overlap

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DescriptorPoolInlineUniformBlockCreateInfo`] or [`MutableDescriptorTypeCreateInfoVALVE`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`DescriptorPoolCreateFlagBits`] values
-  [`pool_sizes`] **must**  be a valid pointer to an array of [`pool_size_count`] valid [`DescriptorPoolSize`] structures
-  [`pool_size_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`DescriptorPoolCreateFlags`]
- [`DescriptorPoolSize`]
- [`StructureType`]
- [`create_descriptor_pool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        