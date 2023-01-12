[VkBufferCollectionConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html) - Structure of general buffer collection constraints

# C Specifications
The [`BufferCollectionConstraintsInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkBufferCollectionConstraintsInfoFUCHSIA {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           minBufferCount;
    uint32_t           maxBufferCount;
    uint32_t           minBufferCountForCamping;
    uint32_t           minBufferCountForDedicatedSlack;
    uint32_t           minBufferCountForSharedSlack;
} VkBufferCollectionConstraintsInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`min_buffer_count`] is the minimum number of buffers available in the collection
- [`max_buffer_count`] is the maximum number of buffers allowed in the collection
- [`min_buffer_count_for_camping`] is the per-participant minimum buffers for camping
- [`min_buffer_count_for_dedicated_slack`] is the per-participant minimum buffers for dedicated slack
- [`min_buffer_count_for_shared_slack`] is the per-participant minimum buffers for shared slack

# Description
Sysmem uses all buffer count parameters in combination to determine the
number of buffers it will allocate.
Sysmem defines buffer count constraints in
`fuchsia.sysmem/constraints.fidl`.*Camping* as referred to by [`min_buffer_count_for_camping`], is the number of
buffers that should be available for the participant that are not for
transient use.
This number of buffers is required for the participant to logically operate.*Slack* as referred to by [`min_buffer_count_for_dedicated_slack`] and
[`min_buffer_count_for_shared_slack`], refers to the number of buffers desired
by participants for optimal performance.
[`min_buffer_count_for_dedicated_slack`] refers to the current participant.
[`min_buffer_count_for_shared_slack`] refers to buffer slack for all
participants in the collection.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`

# Related
- [`fuchsia_buffer_collection`]
- [`BufferConstraintsInfoFUCHSIA`]
- [`ImageConstraintsInfoFUCHSIA`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        